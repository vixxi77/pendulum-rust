use raylib::prelude::*;

fn main() {
    println!("Rust pendulum \n");

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_fps(0, 20);
        d.draw_circle(640/2, 480/2, 50.0, Color::RED);
        d.draw_text("Pendulum simulator", 0, 0, 20, Color::BLACK);
    }
}
