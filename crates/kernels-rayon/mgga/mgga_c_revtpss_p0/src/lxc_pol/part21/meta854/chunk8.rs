//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3231/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3231(t1204: f64, t17948: f64, t1269: f64, t13126: f64, t460: f64, t13147: f64, t1770: f64, t12706: f64, t12717: f64, t13148: f64, t13149: f64, t13150: f64, t17633: f64, t17826: f64, t17879: f64, t17893: f64, t17944: f64, t17952: f64, t1818: f64, t21456: f64, t3746: f64, t3787: f64, t44832: f64, t45385: f64, t45659: f64, t5216: f64, t5481: f64, t58780: f64, t59650: f64, t59784: f64) -> f64 {
    let t59941 = t1204 * t17948;
    let t59945 = t460 * t13126 * t1269;
    let t59948 = t1770 * t13147;
    let t59951 = 0.11853808529283920877e2_f64 * t45659 * t59650 * t58780 + 0.39512695097613069591e1_f64 * t12717 * t17633 * t17944 - 0.39512695097613069591e1_f64 * t1204 * t17879 * t5481 + 0.39512695097613069591e1_f64 * t3746 * t17826 + 0.39512695097613069591e1_f64 * t13148 * t59784 * t13149 - 0.65854491829355115987e0_f64 * t44832 * t1818 + 0.19756347548806534796e1_f64 * t5216 * t3787 - 0.11853808529283920877e2_f64 * t45385 * t17893 - 0.19756347548806534796e1_f64 * t21456 * t12706 + 0.19756347548806534796e1_f64 * t59941 * t17952 + 0.19756347548806534796e1_f64 * t59945 * t17952 + 0.39512695097613069591e1_f64 * t59948 * t13150;
    t59951
}
