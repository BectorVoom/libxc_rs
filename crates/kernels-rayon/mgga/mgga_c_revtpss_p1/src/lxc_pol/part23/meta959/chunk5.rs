//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3226/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3226(t1284: f64, t24698: f64, t12751: f64, t1285: f64, t1287: f64, t1288: f64, t17192: f64, t17958: f64, t20703: f64, t20850: f64, t21448: f64, t21480: f64, t21483: f64, t21484: f64, t21512: f64, t21595: f64, t24931: f64, t25009: f64, t3670: f64, t3746: f64, t3782: f64, t3783: f64, t45769: f64, t45859: f64, t460: f64, t487: f64, t489: f64, t5486: f64, t5487: f64, t59749: f64, t82422: f64, t84203: f64, t84415: f64, t84457: f64) -> f64 {
    let t84859 = t24698 * t1284;
    let t84887 = 0.79025390195226139182e1_f64 * t45859 * t84457 * t21483 - 0.79025390195226139182e1_f64 * t12751 * t21512 * t21595 + 0.65854491829355115987e0_f64 * t84859 * t1288 + 0.65854491829355115987e0_f64 * t3746 * t25009 + 0.39512695097613069591e1_f64 * t45769 * t24931 + 0.39512695097613069592e1_f64 * t3670 * t5486 * t20703 - 0.19756347548806534796e1_f64 * t17192 * t21480 - 0.39512695097613069591e1_f64 * t59749 * t21484 - 0.19756347548806534796e1_f64 * t20850 * t5487 - 0.39512695097613069591e1_f64 * t17958 * t21448 + 0.65854491829355115987e0_f64 * t1285 * t487 * t82422 * t1287 - 0.19756347548806534796e1_f64 * t3782 * t84415 * t3783 + 0.65854491829355115987e0_f64 * t460 * t489 * t84203;
    t84887
}
