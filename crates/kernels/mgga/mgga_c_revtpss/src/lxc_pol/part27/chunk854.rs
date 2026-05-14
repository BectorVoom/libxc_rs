//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 854/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk854<F: Float>(t11213: F, t378: F, t1000: F, t1076: F, t1097: F, t11123: F, t11128: F, t11174: F, t11178: F, t11184: F, t11187: F, t11190: F, t11195: F, t11201: F, t11203: F, t11207: F, t11210: F, t3047: F, t3052: F, t3060: F, t3076: F, t3261: F, t3326: F, t989: F, t995: F) -> (F,) {
    let t11214 = t11213 * t378;
    let t11217 = -0.39512695097613069591e1 * t1076 * t11123 + 0.19756347548806534796e1 * t989 * t3261 - 0.39512695097613069591e1 * t11128 * t1000 - 0.65854491829355115987e0 * t995 * t11174 + 0.39512695097613069591e1 * t1076 * t11178 - 0.19756347548806534796e1 * t3047 * t3076 + 0.19756347548806534796e1 * t995 * t11184 + 0.39512695097613069591e1 * t11187 * t3060 - 0.19756347548806534796e1 * t11190 * t1000 - 0.19756347548806534796e1 * t3052 * t3326 - 0.19756347548806534796e1 * t11195 * t1097 - 0.39512695097613069591e1 * t11201 * t11203 + 0.19756347548806534796e1 * t995 * t11207 - 0.19756347548806534796e1 * t11210 * t1097 - 0.19756347548806534796e1 * t11214 * t1000;
    (t11217,)
}
