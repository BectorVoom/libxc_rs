//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 896/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk896(t30657: f64, t1020: f64, t7605: f64, t1205: f64, t7614: f64, t30228: f64, t601: f64, t30174: f64, t151: f64, t56: f64, t593: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30658 = 0.32155513588552302729e-3_f64 * t30657;
    let t30659 = t7605 * t1020;
    let t30661 = t7614 * t1205;
    let t30663 = t30228 * t601;
    let t30664 = 0.19293308153131381638e-2_f64 * t30663;
    let t30665 = 1.0_f64 / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30670 = 0.36014175219178579057e-1_f64 * t30669;
    let t30671 = t30668 * t606;
    (t30658, t30659, t30661, t30664, t30670, t30671)
}
