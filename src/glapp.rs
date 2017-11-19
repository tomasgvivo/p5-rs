extern crate glutin;
extern crate libc;

use channel;
use color::*;

use self::glutin::GlContext;
use gl;

use std::cell::RefCell;
use std::sync::mpsc;

thread_local! {
    static GLAPP: RefCell<Option<GLApp>> = RefCell::new(None);
}

pub fn listen(rx: mpsc::Receiver<channel::ClosureType>) {
    for boxed_closure in rx {
        boxed_closure();
    };
}

pub fn setup() {
    GLAPP.with(|handle| {
        handle.replace(Some(GLApp::new()));
    });
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.setup();
            }
        });
    }));
}

pub fn poll_events() {
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.poll_events();
            }
        });
    }));
}

pub fn swap_buffers() {
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.swap_buffers();
            }
        });
    }));
}

pub fn background(color: &Color) {
    let &Color{ r, g, b, a } = color;
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

struct GLApp {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow
}

impl GLApp {
    pub fn new() -> GLApp {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("p5-rs sketch")
            .with_dimensions(640, 360);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        GLApp {
            events_loop: events_loop,
            gl_window: gl_window
        }
    }

    pub fn setup(&mut self) {
        unsafe {
            self.gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
    }

    pub fn poll_events(&mut self) {
        let gl_window = &self.gl_window;
        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                // FIXME: send events
                // glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });
    }

    pub fn swap_buffers(&mut self) {
        self.gl_window.swap_buffers().unwrap();
    }
}