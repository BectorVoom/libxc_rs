//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 717/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk717(t9744: f64, t9745: f64, t446: f64, t241: f64, t9577: f64, t9571: f64, t2345: f64, t89: f64, t2594: f64, t9583: f64, t2413: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9746 = t9744 * t9745;
    let t9747 = t446 * t9746;
    let t9749 = t241 * t9577;
    let t9750 = t9749 * t9571;
    let t9752 = t89 * t2345 * t9750;
    let t9754 = t2594 * t9583;
    let t9755 = t446 * t9754;
    let t9757 = t2413 * t713;
    (t9746, t9747, t9749, t9750, t9752, t9754, t9755, t9757)
}
