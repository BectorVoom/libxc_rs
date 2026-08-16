//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 942/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk942(t31362: f64, t7357: f64, t7447: f64, t7503: f64, t30594: f64, t580: f64, t3151: f64, t56: f64, t569: f64, t571: f64, t31276: f64, t7382: f64) -> (f64, f64, f64, f64, f64) {
    let t31363 = t31362 * t7357;
    let t31374 = t7447 * t7503;
    let t31376 = t30594 * t580;
    let t31380 = t3151 * t56 * t569 * t571;
    let t31382 = t31276 * t7382;
    (t31363, t31374, t31376, t31380, t31382)
}
