//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3227/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3227(t1234: f64, t1280: f64, t1281: f64, t1287: f64, t17307: f64, t17864: f64, t17880: f64, t21507: f64, t21551: f64, t21565: f64, t24633: f64, t24948: f64, t24956: f64, t24999: f64, t3670: f64, t3755: f64, t3759: f64, t45385: f64, t45846: f64, t5326: f64, t5459: f64, t59854: f64, t6738: f64, t72270: f64, t72435: f64, t82471: f64, t83108: f64, t83567: f64) -> f64 {
    let t84917 = -0.19756347548806534796e1_f64 * t59854 * t6738 - 0.65854491829355115987e0_f64 * t1234 * t3759 * t24633 + 0.39512695097613069591e1_f64 * t17307 * t21565 - 0.65854491829355115987e0_f64 * t83108 * t1281 - 0.19756347548806534796e1_f64 * t3755 * t82471 * t1287 - 0.19756347548806534796e1_f64 * t5326 * t21551 - 0.39512695097613069591e1_f64 * t45385 * t24956 + 0.19756347548806534796e1_f64 * t72435 * t21507 - 0.19756347548806534796e1_f64 * t17864 * t24999 - 0.19756347548806534796e1_f64 * t17880 * t24999 - 0.19756347548806534796e1_f64 * t72270 * t5459 + 0.65854491829355115987e0_f64 * t45846 * t24948 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t83567;
    t84917
}
