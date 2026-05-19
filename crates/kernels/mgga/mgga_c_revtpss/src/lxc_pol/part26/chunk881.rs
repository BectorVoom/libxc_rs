//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 881/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk881<F: Float>(t11200: F, t378: F, t3059: F, t999: F, t996: F, t3325: F, t1079: F, t3043: F, t3042: F, t993: F, t1000: F, t1076: F, t1097: F, t11123: F, t11128: F, t11174: F, t11178: F, t11184: F, t11187: F, t11190: F, t11195: F, t3047: F, t3052: F, t3060: F, t3076: F, t3261: F, t3326: F, t989: F, t995: F) -> (F, F, F) {
    let t11201 = t11200 * t378;
    let t11202 = t3059 * t999;
    let t11203 = t996 * t11202;
    let t11206 = t999 * t3325;
    let t11207 = t1079 * t11206;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    let t11214 = t11213 * t378;
    let t11217 = -F::cast_from(0.39512695097613069591e1_f64) * t1076 * t11123 + F::cast_from(0.19756347548806534796e1_f64) * t989 * t3261 - F::cast_from(0.39512695097613069591e1_f64) * t11128 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t11174 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t11178 - F::cast_from(0.19756347548806534796e1_f64) * t3047 * t3076 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t11184 + F::cast_from(0.39512695097613069591e1_f64) * t11187 * t3060 - F::cast_from(0.19756347548806534796e1_f64) * t11190 * t1000 - F::cast_from(0.19756347548806534796e1_f64) * t3052 * t3326 - F::cast_from(0.19756347548806534796e1_f64) * t11195 * t1097 - F::cast_from(0.39512695097613069591e1_f64) * t11201 * t11203 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t11207 - F::cast_from(0.19756347548806534796e1_f64) * t11210 * t1097 - F::cast_from(0.19756347548806534796e1_f64) * t11214 * t1000;
    (t11202, t11213, t11217)
}
