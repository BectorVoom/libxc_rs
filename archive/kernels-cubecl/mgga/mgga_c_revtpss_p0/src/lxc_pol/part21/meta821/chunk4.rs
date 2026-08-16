//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3042/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3042<F: Float>(t4746: F, t4980: F, t1082: F, t1087: F, t1089: F, t11202: F, t11782: F, t11788: F, t11940: F, t12111: F, t12124: F, t12157: F, t16152: F, t16381: F, t16402: F, t16529: F, t16544: F, t3204: F, t3287: F, t3291: F, t3313: F, t378: F, t43378: F, t4857: F, t4964: F, t4967: F, t5004: F, t53192: F, t53683: F, t54249: F, t989: F) -> F {
    let t56049 = t4746 * t4980;
    let t56075 = -F::cast_from(0.65854491829355115987e0_f64) * t3287 * t53683 * t1089 - F::cast_from(0.19756347548806534796e1_f64) * t16544 * t12157 - F::cast_from(0.39512695097613069591e1_f64) * t43378 * t4964 - F::cast_from(0.39512695097613069591e1_f64) * t56049 * t12124 + F::cast_from(0.19756347548806534796e1_f64) * t16381 * t3313 + F::cast_from(0.79025390195226139182e1_f64) * t3204 * t3291 * t16152 - F::cast_from(0.65854491829355115987e0_f64) * t4857 * t12111 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t53192 + F::cast_from(0.39512695097613069591e1_f64) * t11788 * t16402 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t378 * t54249 * t1089 - F::cast_from(0.19756347548806534796e1_f64) * t11782 * t4967 + F::cast_from(0.19756347548806534796e1_f64) * t989 * t16529 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t5004 * t11202;
    t56075
}
