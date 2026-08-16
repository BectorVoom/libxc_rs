//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1556/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1556(t3014: f64, t4707: f64, t972: f64, t11450: f64, t11461: f64, t11466: f64, t11554: f64, t15100: f64, t15103: f64, t15104: f64, t15235: f64, t15238: f64, t15242: f64, t15249: f64, t15252: f64, t15255: f64, t2945: f64, t2968: f64, t2987: f64, t3012: f64, t4690: f64, t4712: f64, t965: f64) -> f64 {
    let t15258 = t4707 * t3014;
    let t15259 = t15258 * t972;
    let t15262 = -t15100 + t15103 - 2.0_f64 * t15104 * t2945 + 0.5848223622634646207e0_f64 * t965 * t15235 + 0.32163958997385070134e2_f64 * t2968 * t15238 + 0.2069040516770936012e4_f64 * t11450 * t15242 - 0.23392894490538584828e1_f64 * t11554 * t4690 + 0.34631718211362927518e2_f64 * t11461 * t4712 - 0.23392894490538584828e1_f64 * t2987 * t15249 - 0.11696447245269292414e1_f64 * t2987 * t15252 - 0.10389515463408878255e3_f64 * t11466 * t15255 + 0.34631718211362927518e2_f64 * t3012 * t15259;
    t15262
}
