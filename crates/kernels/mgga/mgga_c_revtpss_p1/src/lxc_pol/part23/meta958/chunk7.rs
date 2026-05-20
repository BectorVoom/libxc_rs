//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3220/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220<F: Float>(t3603: F, t6622: F, t1214: F, t1248: F, t12717: F, t12756: F, t12966: F, t17289: F, t17847: F, t17853: F, t21442: F, t21456: F, t21506: F, t21512: F, t21538: F, t21557: F, t21558: F, t21610: F, t24739: F, t24922: F, t24928: F, t3746: F, t45654: F, t45666: F, t45738: F, t5326: F, t5458: F, t59650: F, t6720: F, t72143: F, t73: F, t82293: F, t84450: F) -> (F, F) {
    let t84645 = t3603 * t6622;
    let t84679 = -F::cast_from(0.11853808529283920877e2_f64) * t17853 * t59650 * t84645 * t1248 - F::cast_from(0.11853808529283920877e2_f64) * t45666 * t24739 * t73 * t5458 + F::cast_from(0.79025390195226139182e1_f64) * t12717 * t21442 * t72143 + F::cast_from(0.39512695097613069592e1_f64) * t12756 * t21512 * t21557 - F::cast_from(0.19756347548806534796e1_f64) * t45738 * t84450 * t21506 + F::cast_from(0.39512695097613069591e1_f64) * t12966 * t24922 - F::cast_from(0.39512695097613069591e1_f64) * t5326 * t21610 - F::cast_from(0.39512695097613069591e1_f64) * t17289 * t6720 - F::cast_from(0.39512695097613069591e1_f64) * t45654 * t82293 * t17847 * t1214 - F::cast_from(0.39512695097613069591e1_f64) * t5326 * t21538 + F::cast_from(0.19756347548806534796e1_f64) * t3746 * t24928 - F::cast_from(0.39512695097613069592e1_f64) * t21456 * t21558;
    (t84645, t84679)
}
