//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3216/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216(t1234: f64, t12751: f64, t1291: f64, t17821: f64, t21507: f64, t21542: f64, t21587: f64, t24698: f64, t25002: f64, t3766: f64, t3769: f64, t3781: f64, t45683: f64, t45738: f64, t45740: f64, t460: f64, t490: f64, t5326: f64, t5465: f64, t5466: f64, t5481: f64, t6587: f64, t6695: f64, t72343: f64, t72732: f64, t82293: f64, t82321: f64, t83232: f64, t84487: f64) -> f64 {
    let t84541 = -0.39512695097613069592e1_f64 * t12751 * t84487 * t5465 + 0.65854491829355115987e0_f64 * t83232 * t490 + 0.65854491829355115987e0_f64 * t24698 * t1291 - 0.39512695097613069591e1_f64 * t45683 * t25002 - 0.39512695097613069591e1_f64 * t12751 * t82321 * t3769 - 0.11853808529283920877e2_f64 * t72343 * t21587 + 0.19756347548806534796e1_f64 * t72732 * t21507 - 0.65854491829355115987e0_f64 * t45738 * t82293 * t45740 - 0.19756347548806534796e1_f64 * t1234 * t17821 * t6587 - 0.19756347548806534796e1_f64 * t5326 * t21542 + 0.39512695097613069592e1_f64 * t460 * t3766 * t6695 * t5466 - 0.19756347548806534796e1_f64 * t460 * t3781 * t6695 * t5481;
    t84541
}
