//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3026/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3026<F: Float>(t15780: F, t3302: F, t1024: F, t1043: F, t1082: F, t1089: F, t11788: F, t12073: F, t12122: F, t12127: F, t12149: F, t15604: F, t15837: F, t16432: F, t16433: F, t16436: F, t16440: F, t16449: F, t16482: F, t3059: F, t3204: F, t3278: F, t354: F, t43432: F, t43453: F, t43528: F, t4772: F, t4781: F, t53273: F, t54360: F, t54931: F, t54936: F) -> (F, F) {
    let t55550 = t15780 * t3302;
    let t55562 = -F::cast_from(0.39512695097613069591e1_f64) * t12122 * t16432 * t54931 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t16432 * t54936 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1082 * t53273 - F::cast_from(0.79025390195226139182e1_f64) * t43432 * t16433 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t15837 * t1043 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t4781 * t354 * t54360 + F::cast_from(0.19756347548806534796e1_f64) * t3278 * t16440 + F::cast_from(0.39512695097613069591e1_f64) * t43453 * t16436 + F::cast_from(0.39512695097613069591e1_f64) * t43528 * t16436 + F::cast_from(0.39512695097613069591e1_f64) * t12127 * t55550 * t15604 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t12073 * t4772 + F::cast_from(0.79025390195226139182e1_f64) * t11788 * t16482 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t16449 * t3059;
    (t55550, t55562)
}
