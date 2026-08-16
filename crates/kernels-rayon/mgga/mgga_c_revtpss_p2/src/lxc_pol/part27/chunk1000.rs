//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1000/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1000(t12108: f64, t12172: f64, t1079: f64, t1096: f64, t3059: f64, t1073: f64, t1076: f64, t1097: f64, t11220: f64, t11224: f64, t11902: f64, t12034: f64, t12040: f64, t12043: f64, t3043: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3264: f64, t3271: f64, t3326: f64, t342: f64, t386: f64, t995: f64) -> f64 {
    let t12173 = t12108 + t12172;
    let t12174 = t1079 * t12173;
    let t12177 = t3059 * t1096;
    let t12178 = t1079 * t12177;
    let t12189 = 0.39512695097613069591e1_f64 * t3047 * t3067 - 0.39512695097613069591e1_f64 * t11220 * t1097 + 0.39512695097613069591e1_f64 * t11224 * t3060 - 0.19756347548806534796e1_f64 * t3264 * t3326 + 0.39512695097613069591e1_f64 * t3063 * t3067 + 0.65854491829355115987e0_f64 * t342 * t12034 + 0.39512695097613069591e1_f64 * t3052 * t3271 - 0.39512695097613069591e1_f64 * t995 * t12040 + 0.39512695097613069591e1_f64 * t3058 * t12043 - 0.65854491829355115987e0_f64 * t1076 * t12174 - 0.39512695097613069591e1_f64 * t3058 * t12178 - 0.19756347548806534796e1_f64 * t3063 * t3076 + 0.39512695097613069591e1_f64 * t3264 * t3271 + 0.65854491829355115987e0_f64 * t11902 * t386 + 0.19756347548806534796e1_f64 * t3043 * t1073;
    t12189
}
