//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3042/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3042(t4746: f64, t4980: f64, t1082: f64, t1087: f64, t1089: f64, t11202: f64, t11782: f64, t11788: f64, t11940: f64, t12111: f64, t12124: f64, t12157: f64, t16152: f64, t16381: f64, t16402: f64, t16529: f64, t16544: f64, t3204: f64, t3287: f64, t3291: f64, t3313: f64, t378: f64, t43378: f64, t4857: f64, t4964: f64, t4967: f64, t5004: f64, t53192: f64, t53683: f64, t54249: f64, t989: f64) -> f64 {
    let t56049 = t4746 * t4980;
    let t56075 = -0.65854491829355115987e0_f64 * t3287 * t53683 * t1089 - 0.19756347548806534796e1_f64 * t16544 * t12157 - 0.39512695097613069591e1_f64 * t43378 * t4964 - 0.39512695097613069591e1_f64 * t56049 * t12124 + 0.19756347548806534796e1_f64 * t16381 * t3313 + 0.79025390195226139182e1_f64 * t3204 * t3291 * t16152 - 0.65854491829355115987e0_f64 * t4857 * t12111 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t53192 + 0.39512695097613069591e1_f64 * t11788 * t16402 + 0.65854491829355115987e0_f64 * t1087 * t378 * t54249 * t1089 - 0.19756347548806534796e1_f64 * t11782 * t4967 + 0.19756347548806534796e1_f64 * t989 * t16529 - 0.39512695097613069591e1_f64 * t11940 * t5004 * t11202;
    t56075
}
