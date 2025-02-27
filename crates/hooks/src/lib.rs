//! # Freya Hooks
//! A collection of hooks to be used in Freya.

mod animation;
mod rope_editor;
mod shader_uniforms;
mod text_editor;

mod use_accessibility;
mod use_animation;
mod use_animation_transition;
mod use_canvas;
mod use_editable;
mod use_focus;
mod use_node;
mod use_platform;
mod use_theme;

#[cfg(feature = "use_camera")]
mod use_camera;

pub use animation::*;
pub use rope_editor::*;
pub use shader_uniforms::*;
pub use text_editor::*;

pub use use_accessibility::*;
pub use use_animation::*;
pub use use_animation_transition::*;
pub use use_canvas::*;
pub use use_editable::*;
pub use use_focus::*;
pub use use_node::*;
pub use use_platform::*;
pub use use_theme::*;

#[cfg(feature = "use_camera")]
pub use use_camera::*;
