//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3228/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3228<F: Float>(t17191: F, t3566: F, t16756: F, t3302: F, t12719: F, t12751: F, t12756: F, t1280: F, t12966: F, t13134: F, t13142: F, t13143: F, t16695: F, t16696: F, t16697: F, t16776: F, t17853: F, t17855: F, t1822: F, t3670: F, t45634: F, t45718: F, t45726: F, t5326: F, t5465: F, t56530: F, t56555: F, t57536: F, t58760: F, t59699: F, t59784: F) -> (F, F) {
    let t59817 = t3566 * t17191;
    let t59824 = t16756 * t3302;
    let t59833 = -F::cast_from(0.39512695097613069591e1_f64) * t12751 * t59699 * t5465 - F::cast_from(0.39512695097613069591e1_f64) * t13142 * t59784 * t13143 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t56555 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t56530 + F::cast_from(0.39512695097613069591e1_f64) * t12966 * t16776 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t13134 - F::cast_from(0.11853808529283920877e2_f64) * t17853 * t57536 * t17855 + F::cast_from(0.39512695097613069591e1_f64) * t59817 * t12719 + F::cast_from(0.39512695097613069591e1_f64) * t45718 * t16697 + F::cast_from(0.39512695097613069591e1_f64) * t45634 * t16697 + F::cast_from(0.39512695097613069591e1_f64) * t12756 * t59824 * t16696 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t16695 * t58760 + F::cast_from(0.65854491829355115987e0_f64) * t45726 * t1822;
    (t59824, t59833)
}
