//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1075/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1075(t11213: f64, t378: f64, t1000: f64, t1076: f64, t1097: f64, t11123: f64, t11128: f64, t11174: f64, t11178: f64, t11184: f64, t11187: f64, t11190: f64, t11195: f64, t11201: f64, t11203: f64, t11207: f64, t11210: f64, t3047: f64, t3052: f64, t3060: f64, t3076: f64, t3261: f64, t3326: f64, t989: f64, t995: f64) -> (f64, f64) {
    let t11214 = t11213 * t378;
    let t11217 = -0.39512695097613069591e1_f64 * t1076 * t11123 + 0.19756347548806534796e1_f64 * t989 * t3261 - 0.39512695097613069591e1_f64 * t11128 * t1000 - 0.65854491829355115987e0_f64 * t995 * t11174 + 0.39512695097613069591e1_f64 * t1076 * t11178 - 0.19756347548806534796e1_f64 * t3047 * t3076 + 0.19756347548806534796e1_f64 * t995 * t11184 + 0.39512695097613069591e1_f64 * t11187 * t3060 - 0.19756347548806534796e1_f64 * t11190 * t1000 - 0.19756347548806534796e1_f64 * t3052 * t3326 - 0.19756347548806534796e1_f64 * t11195 * t1097 - 0.39512695097613069591e1_f64 * t11201 * t11203 + 0.19756347548806534796e1_f64 * t995 * t11207 - 0.19756347548806534796e1_f64 * t11210 * t1097 - 0.19756347548806534796e1_f64 * t11214 * t1000;
    (t11214, t11217)
}
