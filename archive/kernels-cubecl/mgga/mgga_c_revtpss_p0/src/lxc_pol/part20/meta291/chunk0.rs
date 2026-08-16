//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1159/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1159<F: Float>(t12108: F, t12172: F, t1079: F, t1096: F, t3059: F, t1073: F, t1076: F, t1097: F, t11220: F, t11224: F, t11902: F, t12034: F, t12040: F, t12043: F, t3043: F, t3047: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3264: F, t3271: F, t3326: F, t342: F, t386: F, t995: F) -> (F, F, F, F) {
    let t12173 = t12108 + t12172;
    let t12174 = t1079 * t12173;
    let t12177 = t3059 * t1096;
    let t12178 = t1079 * t12177;
    let t12189 = F::cast_from(0.39512695097613069591e1_f64) * t3047 * t3067 - F::cast_from(0.39512695097613069591e1_f64) * t11220 * t1097 + F::cast_from(0.39512695097613069591e1_f64) * t11224 * t3060 - F::cast_from(0.19756347548806534796e1_f64) * t3264 * t3326 + F::cast_from(0.39512695097613069591e1_f64) * t3063 * t3067 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t12034 + F::cast_from(0.39512695097613069591e1_f64) * t3052 * t3271 - F::cast_from(0.39512695097613069591e1_f64) * t995 * t12040 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t12043 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t12174 - F::cast_from(0.39512695097613069591e1_f64) * t3058 * t12178 - F::cast_from(0.19756347548806534796e1_f64) * t3063 * t3076 + F::cast_from(0.39512695097613069591e1_f64) * t3264 * t3271 + F::cast_from(0.65854491829355115987e0_f64) * t11902 * t386 + F::cast_from(0.19756347548806534796e1_f64) * t3043 * t1073;
    (t12173, t12174, t12178, t12189)
}
