//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3083/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3083<F: Float>(t56354: F, t56375: F, t1770: F, t3727: F, t1210: F, t1211: F, t12606: F, t12622: F, t12630: F, t12646: F, t12695: F, t1277: F, t1295: F, t13170: F, t13173: F, t13177: F, t1775: F, t17964: F, t17973: F, t17974: F, t17988: F, t18090: F, t1829: F, t21389: F, t34934: F, t34964: F, t3572: F, t3575: F, t3732: F, t3790: F, t45464: F, t45568: F, t5220: F, t5245: F, t5423: F, t56294: F, t56303: F, t56310: F, t56314: F, t56315: F, t56327: F, t56332: F) -> (F, F) {
    let t56376 = t56354 + t56375;
    let t56384 = t1770 * t3727;
    let t56390 = -F::cast_from(0.65854491829355115987e0_f64) * t45568 * t1775 - F::cast_from(0.79025390195226139182e1_f64) * t56294 * t17988 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t5245 * t3790 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t13170 - F::cast_from(0.19756347548806534796e1_f64) * t56303 * t1295 - F::cast_from(0.19756347548806534796e1_f64) * t3732 * t17964 + F::cast_from(0.39512695097613069591e1_f64) * t13177 * t5423 - F::cast_from(0.79025390195226139182e1_f64) * t17973 * t56310 * t3575 - F::cast_from(0.11853808529283920877e2_f64) * t56314 * t56315 * t12695 - F::cast_from(0.39512695097613069591e1_f64) * t17973 * t17974 * t12606 - F::cast_from(0.19756347548806534796e1_f64) * t45464 * t1829 - F::cast_from(0.39512695097613069591e1_f64) * t17973 * t34964 * t12646 - F::cast_from(0.11853808529283920877e2_f64) * t56327 * t34934 * t12646 - F::cast_from(0.39512695097613069591e1_f64) * t56332 * t12630 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1211 * t56376 - F::cast_from(0.65854491829355115987e0_f64) * t5220 * t12622 - F::cast_from(0.19756347548806534796e1_f64) * t3572 * t18090 - F::cast_from(0.19756347548806534796e1_f64) * t56384 * t1295 + F::cast_from(0.79025390195226139182e1_f64) * t17973 * t21389 * t13173;
    (t56376, t56390)
}
