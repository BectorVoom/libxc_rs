//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 937/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk937(t31362: f64, t7357: f64, t7447: f64, t7503: f64, t30594: f64, t580: f64, t3151: f64, t56: f64, t569: f64, t571: f64, t31276: f64, t7382: f64) -> (f64, f64, f64, f64, f64) {
    let t31363 = t31362 * t7357;
    let t31374 = t7447 * t7503;
    let t31376 = t30594 * t580;
    let t31377 = 1309.0_f64 / 5184.0_f64 * t31376;
    let t31380 = t3151 * t56 * t569 * t571;
    let t31381 = 455.0_f64 / 1296.0_f64 * t31380;
    let t31382 = t31276 * t7382;
    (t31363, t31374, t31377, t31381, t31382)
}
