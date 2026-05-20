//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3025/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3025<F: Float>(t1072: F, t3057: F, t1647: F, t3259: F, t1071: F, t15669: F, t15654: F, t12050: F, t15907: F, t16076: F, t3153: F, t1024: F, t1043: F, t1082: F, t1087: F, t1089: F, t11173: F, t11940: F, t12097: F, t12122: F, t12127: F, t16237: F, t16432: F, t16458: F, t16461: F, t16559: F, t16566: F, t3223: F, t43443: F, t43598: F, t4983: F, t4992: F, t4998: F, t5004: F, t53089: F, t53516: F, t53909: F, t54130: F) -> (F, F, F, F, F, F) {
    let t55458 = t3057 * t1072;
    let t55461 = t1647 * t3259;
    let t55464 = t15669 * t1071;
    let t55475 = t15654 * t1071;
    let t55499 = t15907 * t12050;
    let t55517 = t16076 * t3153;
    let t55524 = -F::cast_from(0.11853808529283920877e2_f64) * t11940 * t1082 * t53089 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t16237 * t1043 * t1089 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t16432 * t54130 - F::cast_from(0.11853808529283920877e2_f64) * t16559 * t55499 * t53909 - F::cast_from(0.39512695097613069591e1_f64) * t3223 * t16461 + F::cast_from(0.19756347548806534796e1_f64) * t12097 * t4992 + F::cast_from(0.19756347548806534796e1_f64) * t16566 * t55499 * t53516 + F::cast_from(0.79025390195226139182e1_f64) * t43443 * t16458 + F::cast_from(0.79025390195226139182e1_f64) * t43598 * t16458 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t5004 * t11173 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t55517 * t4983 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t55517 * t4998;
    (t55458, t55461, t55464, t55475, t55499, t55524)
}
