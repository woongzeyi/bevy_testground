// Modified from https://github.com/bevyengine/bevy/blob/release-0.13.0/examples/input/mouse_input_events.rs
//! Prints all mouse events to the console.

use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // CHANGEME Comment and uncomment the line below on Bevy 0.13 to try out the effects
        .add_systems(Startup, camera_setup)
        .add_systems(Update, print_mouse_events_system)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    // mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    // mut cursor_moved_events: EventReader<CursorMoved>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    // mut touchpad_magnify_events: EventReader<TouchpadMagnify>,
    // mut touchpad_rotate_events: EventReader<TouchpadRotate>,
) {
    // for event in mouse_button_input_events.read() {
    //     info!("{:?}", event);
    // }

    for event in mouse_motion_events.read() {
        info!("{:?}", event);
    }

    // for event in cursor_moved_events.read() {
    //     info!("{:?}", event);
    // }

    // for event in mouse_wheel_events.read() {
    //     info!("{:?}", event);
    // }

    // // This event will only fire on macOS
    // for event in touchpad_magnify_events.read() {
    //     info!("{:?}", event);
    // }

    // // This event will only fire on macOS
    // for event in touchpad_rotate_events.read() {
    //     info!("{:?}", event);
    // }
}
