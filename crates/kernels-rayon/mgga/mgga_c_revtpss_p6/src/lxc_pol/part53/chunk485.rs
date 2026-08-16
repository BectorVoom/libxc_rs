//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 485/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk485(t340: f64, t992: f64, t338: f64, t378: f64, t1071: f64, t994: f64, t2846: f64, t221: f64, t346: f64, t696: f64, t345: f64, t1003: f64, t1007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3056 = 1.0_f64 / t992 / t340;
    let t3057 = t338 * t3056;
    let t3058 = t3057 * t378;
    let t3063 = t994 * t1071;
    let t3070 = 0.19755555555555555556e-1_f64 * t2846;
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / 432.0_f64;
    let t3086 = t1003 * t1007;
    (t3056, t3057, t3058, t3063, t3070, t3080, t3082, t3086)
}
