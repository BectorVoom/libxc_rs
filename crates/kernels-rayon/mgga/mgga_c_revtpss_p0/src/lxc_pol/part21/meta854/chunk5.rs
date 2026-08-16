//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3228/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3228(t17191: f64, t3566: f64, t16756: f64, t3302: f64, t12719: f64, t12751: f64, t12756: f64, t1280: f64, t12966: f64, t13134: f64, t13142: f64, t13143: f64, t16695: f64, t16696: f64, t16697: f64, t16776: f64, t17853: f64, t17855: f64, t1822: f64, t3670: f64, t45634: f64, t45718: f64, t45726: f64, t5326: f64, t5465: f64, t56530: f64, t56555: f64, t57536: f64, t58760: f64, t59699: f64, t59784: f64) -> (f64, f64) {
    let t59817 = t3566 * t17191;
    let t59824 = t16756 * t3302;
    let t59833 = -0.39512695097613069591e1_f64 * t12751 * t59699 * t5465 - 0.39512695097613069591e1_f64 * t13142 * t59784 * t13143 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t56555 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t56530 + 0.39512695097613069591e1_f64 * t12966 * t16776 - 0.19756347548806534796e1_f64 * t5326 * t13134 - 0.11853808529283920877e2_f64 * t17853 * t57536 * t17855 + 0.39512695097613069591e1_f64 * t59817 * t12719 + 0.39512695097613069591e1_f64 * t45718 * t16697 + 0.39512695097613069591e1_f64 * t45634 * t16697 + 0.39512695097613069591e1_f64 * t12756 * t59824 * t16696 + 0.19756347548806534796e1_f64 * t12756 * t16695 * t58760 + 0.65854491829355115987e0_f64 * t45726 * t1822;
    (t59824, t59833)
}
