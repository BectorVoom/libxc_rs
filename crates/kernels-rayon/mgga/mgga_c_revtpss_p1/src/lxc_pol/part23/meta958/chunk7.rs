//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3220/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220(t3603: f64, t6622: f64, t1214: f64, t1248: f64, t12717: f64, t12756: f64, t12966: f64, t17289: f64, t17847: f64, t17853: f64, t21442: f64, t21456: f64, t21506: f64, t21512: f64, t21538: f64, t21557: f64, t21558: f64, t21610: f64, t24739: f64, t24922: f64, t24928: f64, t3746: f64, t45654: f64, t45666: f64, t45738: f64, t5326: f64, t5458: f64, t59650: f64, t6720: f64, t72143: f64, t73: f64, t82293: f64, t84450: f64) -> (f64, f64) {
    let t84645 = t3603 * t6622;
    let t84679 = -0.11853808529283920877e2_f64 * t17853 * t59650 * t84645 * t1248 - 0.11853808529283920877e2_f64 * t45666 * t24739 * t73 * t5458 + 0.79025390195226139182e1_f64 * t12717 * t21442 * t72143 + 0.39512695097613069592e1_f64 * t12756 * t21512 * t21557 - 0.19756347548806534796e1_f64 * t45738 * t84450 * t21506 + 0.39512695097613069591e1_f64 * t12966 * t24922 - 0.39512695097613069591e1_f64 * t5326 * t21610 - 0.39512695097613069591e1_f64 * t17289 * t6720 - 0.39512695097613069591e1_f64 * t45654 * t82293 * t17847 * t1214 - 0.39512695097613069591e1_f64 * t5326 * t21538 + 0.19756347548806534796e1_f64 * t3746 * t24928 - 0.39512695097613069592e1_f64 * t21456 * t21558;
    (t84645, t84679)
}
