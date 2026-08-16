//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 919/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk919(t30594: f64, t580: f64, t3151: f64, t56: f64, t569: f64, t571: f64, t31276: f64, t7382: f64, t1072: f64, t429: f64, t7507: f64, t7512: f64) -> (f64, f64, f64, f64) {
    let t31376 = t30594 * t580;
    let t31380 = t3151 * t56 * t569 * t571;
    let t31382 = t31276 * t7382;
    let t31386 = t7507 * t7512 * t429 * t1072;
    (t31376, t31380, t31382, t31386)
}
