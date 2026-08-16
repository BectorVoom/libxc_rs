//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3029/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3029<F: Float>(t15893: F, t3153: F, t16551: F, t989: F, t1043: F, t1089: F, t12097: F, t12149: F, t15957: F, t16152: F, t16410: F, t16443: F, t16479: F, t16523: F, t16534: F, t16555: F, t16577: F, t19502: F, t3043: F, t3223: F, t3287: F, t43438: F, t43450: F, t43520: F, t43524: F, t4964: F, t4976: F, t4988: F, t5012: F, t53340: F, t53506: F, t54026: F, t55499: F) -> (F, F) {
    let t55612 = t15893 * t3153;
    let t55632 = t989 * t16551;
    let t55643 = F::cast_from(0.79025390195226139182e1_f64) * t12149 * t16152 * t1043 * t1089 + F::cast_from(0.79025390195226139182e1_f64) * t43438 * t55612 * t19502 + F::cast_from(0.19756347548806534796e1_f64) * t12097 * t4988 - F::cast_from(0.11853808529283920877e2_f64) * t43520 * t55499 * t53506 + F::cast_from(0.11853808529283920877e2_f64) * t43524 * t55499 * t53340 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t54026 * t4976 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t15957 * t16577 - F::cast_from(0.39512695097613069591e1_f64) * t16523 * t16534 + F::cast_from(0.11853808529283920877e2_f64) * t55632 * t16555 - F::cast_from(0.19756347548806534796e1_f64) * t43450 * t4964 - F::cast_from(0.19756347548806534796e1_f64) * t3223 * t16479 + F::cast_from(0.19756347548806534796e1_f64) * t3043 * t5012 + F::cast_from(0.79025390195226139182e1_f64) * t16410 * t16443;
    (t55612, t55643)
}
