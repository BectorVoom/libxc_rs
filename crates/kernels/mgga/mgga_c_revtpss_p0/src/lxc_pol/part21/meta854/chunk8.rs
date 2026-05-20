//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3231/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3231<F: Float>(t1204: F, t17948: F, t1269: F, t13126: F, t460: F, t13147: F, t1770: F, t12706: F, t12717: F, t13148: F, t13149: F, t13150: F, t17633: F, t17826: F, t17879: F, t17893: F, t17944: F, t17952: F, t1818: F, t21456: F, t3746: F, t3787: F, t44832: F, t45385: F, t45659: F, t5216: F, t5481: F, t58780: F, t59650: F, t59784: F) -> F {
    let t59941 = t1204 * t17948;
    let t59945 = t460 * t13126 * t1269;
    let t59948 = t1770 * t13147;
    let t59951 = F::cast_from(0.11853808529283920877e2_f64) * t45659 * t59650 * t58780 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t17633 * t17944 - F::cast_from(0.39512695097613069591e1_f64) * t1204 * t17879 * t5481 + F::cast_from(0.39512695097613069591e1_f64) * t3746 * t17826 + F::cast_from(0.39512695097613069591e1_f64) * t13148 * t59784 * t13149 - F::cast_from(0.65854491829355115987e0_f64) * t44832 * t1818 + F::cast_from(0.19756347548806534796e1_f64) * t5216 * t3787 - F::cast_from(0.11853808529283920877e2_f64) * t45385 * t17893 - F::cast_from(0.19756347548806534796e1_f64) * t21456 * t12706 + F::cast_from(0.19756347548806534796e1_f64) * t59941 * t17952 + F::cast_from(0.19756347548806534796e1_f64) * t59945 * t17952 + F::cast_from(0.39512695097613069591e1_f64) * t59948 * t13150;
    t59951
}
