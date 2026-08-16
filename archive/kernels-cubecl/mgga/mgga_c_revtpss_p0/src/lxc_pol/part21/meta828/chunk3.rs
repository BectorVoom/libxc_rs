//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3087/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3087<F: Float>(t459: F, t56456: F, t56477: F, t1215: F, t12630: F, t12641: F, t1271: F, t1274: F, t1277: F, t13173: F, t13174: F, t13182: F, t17331: F, t17964: F, t17968: F, t17975: F, t17986: F, t17988: F, t18084: F, t18090: F, t18103: F, t3552: F, t3556: F, t3561: F, t3567: F, t3568: F, t3569: F, t3572: F, t3729: F, t3732: F, t3738: F, t495: F, t5216: F, t5251: F, t5414: F, t5497: F, t56315: F, t56393: F, t56396: F, t56413: F, t56416: F, t56419: F, t56432: F) -> (F, F) {
    let t56479 = (t56456 + t56477) * t459;
    let t56484 = -F::cast_from(0.39512695097613069591e1_f64) * t12641 * t18103 - F::cast_from(0.39512695097613069591e1_f64) * t56393 * t12630 - F::cast_from(0.19756347548806534796e1_f64) * t56396 * t1215 - F::cast_from(0.19756347548806534796e1_f64) * t3556 * t18090 + F::cast_from(0.19756347548806534796e1_f64) * t3572 * t18084 + F::cast_from(0.11853808529283920877e2_f64) * t17986 * t56315 * t13173 - F::cast_from(0.11853808529283920877e2_f64) * t3732 * t17968 - F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1277 * t5497 * t3568 - F::cast_from(0.19756347548806534796e1_f64) * t56413 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t56416 * t3569 - F::cast_from(0.79025390195226139182e1_f64) * t56419 * t17988 + F::cast_from(0.19756347548806534796e1_f64) * t5216 * t3729 - F::cast_from(0.19756347548806534796e1_f64) * t3561 * t17964 - F::cast_from(0.11853808529283920877e2_f64) * t1274 * t13182 * t5497 * t3738 + F::cast_from(0.19756347548806534796e1_f64) * t3552 * t5414 - F::cast_from(0.79025390195226139182e1_f64) * t56432 * t17975 - F::cast_from(0.39512695097613069591e1_f64) * t5251 * t13174 + F::cast_from(0.65854491829355115987e0_f64) * t56479 * t495 + F::cast_from(0.19756347548806534796e1_f64) * t17331 * t1271;
    (t56479, t56484)
}
