//! Branch-free piecewise selection, matching libxc's `my_piecewise*` macros.
//!
//! The CubeCL originals use `select()`, which evaluates both arms. Rust's
//! `if/else` over already-evaluated arguments does the same work in the same
//! order, so results are identical.

/// libxc `my_piecewise3(c, x1, x2)`.
#[inline(always)]
pub fn piecewise3(cond: bool, val_true: f64, val_false: f64) -> f64 {
    if cond { val_true } else { val_false }
}

/// libxc `my_piecewise5(c1, x1, c2, x2, x3)`.
#[inline(always)]
pub fn piecewise5(c1: bool, v1: f64, c2: bool, v2: f64, v_else: f64) -> f64 {
    if c1 { v1 } else if c2 { v2 } else { v_else }
}

/// libxc `Heaviside(x)`.
#[inline(always)]
#[allow(non_snake_case)]
pub fn Heaviside(x: f64) -> f64 {
    if x >= 0.0 { 1.0 } else { 0.0 }
}
