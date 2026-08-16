//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1135/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1135(t22496: f64, t25082: f64, t36970: f64, t8594: f64, t9593: f64, t28196: f64, t28198: f64, t28166: f64, t8567: f64, t28168: f64, t32117: f64, t7898: f64) -> (f64, f64, f64, f64) {
    let t125491 = 3.0_f64 * t25082 * t36970 * t22496;
    let t125492 = t8594 * t9593;
    let t125495 = 2.0_f64 * t28196 * t125492 * t28198;
    let t125496 = t8567 * t28166;
    let t125497 = t125496 * t28168;
    let t125499 = t7898 * t32117;
    (t125491, t125495, t125497, t125499)
}
