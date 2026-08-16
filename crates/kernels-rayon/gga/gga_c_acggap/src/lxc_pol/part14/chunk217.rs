//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 217/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk217(t257: f64, t739: f64, t249: f64, t62: f64, t70: f64, t729: f64, t31: f64, t4: f64, t668: f64, t132: f64, t200: f64, t220: f64, t721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t740 = t739 * t257;
    let t743 = t249 * t249;
    let t744 = 1.0_f64 / t743;
    let t745 = t62 * t744;
    let t746 = t70 * t70;
    let t747 = 1.0_f64 / t746;
    let t748 = t729 * t747;
    let t752 = t4 * t668 * t31;
    let t753 = 0.14764627977777777777e-2_f64 * t752;
    let t754 = t132 * t200;
    let t756 = t721 * t754 * t220;
    (t740, t743, t744, t745, t746, t747, t748, t753, t754, t756)
}
