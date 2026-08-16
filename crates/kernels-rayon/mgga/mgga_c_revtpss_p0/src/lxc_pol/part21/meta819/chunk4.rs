//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3025/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3025(t1072: f64, t3057: f64, t1647: f64, t3259: f64, t1071: f64, t15669: f64, t15654: f64, t12050: f64, t15907: f64, t16076: f64, t3153: f64, t1024: f64, t1043: f64, t1082: f64, t1087: f64, t1089: f64, t11173: f64, t11940: f64, t12097: f64, t12122: f64, t12127: f64, t16237: f64, t16432: f64, t16458: f64, t16461: f64, t16559: f64, t16566: f64, t3223: f64, t43443: f64, t43598: f64, t4983: f64, t4992: f64, t4998: f64, t5004: f64, t53089: f64, t53516: f64, t53909: f64, t54130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55458 = t3057 * t1072;
    let t55461 = t1647 * t3259;
    let t55464 = t15669 * t1071;
    let t55475 = t15654 * t1071;
    let t55499 = t15907 * t12050;
    let t55517 = t16076 * t3153;
    let t55524 = -0.11853808529283920877e2_f64 * t11940 * t1082 * t53089 + 0.19756347548806534796e1_f64 * t1087 * t16237 * t1043 * t1089 + 0.19756347548806534796e1_f64 * t12127 * t16432 * t54130 - 0.11853808529283920877e2_f64 * t16559 * t55499 * t53909 - 0.39512695097613069591e1_f64 * t3223 * t16461 + 0.19756347548806534796e1_f64 * t12097 * t4992 + 0.19756347548806534796e1_f64 * t16566 * t55499 * t53516 + 0.79025390195226139182e1_f64 * t43443 * t16458 + 0.79025390195226139182e1_f64 * t43598 * t16458 - 0.65854491829355115987e0_f64 * t1024 * t5004 * t11173 - 0.39512695097613069591e1_f64 * t12122 * t55517 * t4983 + 0.19756347548806534796e1_f64 * t12127 * t55517 * t4998;
    (t55458, t55461, t55464, t55475, t55499, t55524)
}
