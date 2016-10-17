#![cfg(target_os = "macos")]

use std::convert::From;
use std::os::raw::c_void;
use cocoa::appkit::NSApplicationActivationPolicy;
use api::cocoa::IdRef;
use {Window, WindowBuilder};

/// Additional methods on `Window` that are specific to MacOS.
pub trait WindowExt {
    /// Returns a pointer to the cocoa `NSWindow` that is used by this window.
    ///
    /// The pointer will become invalid when the glutin `Window` is destroyed.
    fn get_nswindow(&self) -> *mut c_void;

    /// Returns the id of the cocoa `NSView` that is used by this window.
    ///
    ///
    fn get_native_nsview(&self) -> &IdRef;
}

impl WindowExt for Window {
    #[inline]
    fn get_nswindow(&self) -> *mut c_void {
        self.window.platform_window() as *mut c_void
    }

    #[inline]
    fn get_native_nsview(&self) -> &IdRef {
        &self.window.get_native_view()
    }
}

/// Corresponds to `NSApplicationActivationPolicy`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationPolicy {
    /// Corresponds to `NSApplicationActivationPolicyRegular`.
    Regular,
    /// Corresponds to `NSApplicationActivationPolicyAccessory`.
    Accessory,
    /// Corresponds to `NSApplicationActivationPolicyProhibited`.
    Prohibited,
}

impl Default for ActivationPolicy {
    fn default() -> Self {
        ActivationPolicy::Regular
    }
}

impl From<ActivationPolicy> for NSApplicationActivationPolicy {
    fn from(activation_policy: ActivationPolicy) -> Self {
        match activation_policy {
            ActivationPolicy::Regular =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            ActivationPolicy::Accessory =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
            ActivationPolicy::Prohibited =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyProhibited,
        }
    }
}

/// Additional methods on `WindowBuilder` that are specific to MacOS.
pub trait WindowBuilderExt {
    fn with_activation_policy(mut self, activation_policy: ActivationPolicy) -> WindowBuilder;
}

impl WindowBuilderExt for WindowBuilder {
    /// Sets the activation policy for the window being built
    #[inline]
    fn with_activation_policy(mut self, activation_policy: ActivationPolicy) -> WindowBuilder {
        self.platform_specific.activation_policy = activation_policy;
        self
    }
}
