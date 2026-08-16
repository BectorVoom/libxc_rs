//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3029/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3029(t15893: f64, t3153: f64, t16551: f64, t989: f64, t1043: f64, t1089: f64, t12097: f64, t12149: f64, t15957: f64, t16152: f64, t16410: f64, t16443: f64, t16479: f64, t16523: f64, t16534: f64, t16555: f64, t16577: f64, t19502: f64, t3043: f64, t3223: f64, t3287: f64, t43438: f64, t43450: f64, t43520: f64, t43524: f64, t4964: f64, t4976: f64, t4988: f64, t5012: f64, t53340: f64, t53506: f64, t54026: f64, t55499: f64) -> (f64, f64) {
    let t55612 = t15893 * t3153;
    let t55632 = t989 * t16551;
    let t55643 = 0.79025390195226139182e1_f64 * t12149 * t16152 * t1043 * t1089 + 0.79025390195226139182e1_f64 * t43438 * t55612 * t19502 + 0.19756347548806534796e1_f64 * t12097 * t4988 - 0.11853808529283920877e2_f64 * t43520 * t55499 * t53506 + 0.11853808529283920877e2_f64 * t43524 * t55499 * t53340 - 0.19756347548806534796e1_f64 * t3287 * t54026 * t4976 + 0.39512695097613069591e1_f64 * t12149 * t15957 * t16577 - 0.39512695097613069591e1_f64 * t16523 * t16534 + 0.11853808529283920877e2_f64 * t55632 * t16555 - 0.19756347548806534796e1_f64 * t43450 * t4964 - 0.19756347548806534796e1_f64 * t3223 * t16479 + 0.19756347548806534796e1_f64 * t3043 * t5012 + 0.79025390195226139182e1_f64 * t16410 * t16443;
    (t55612, t55643)
}
