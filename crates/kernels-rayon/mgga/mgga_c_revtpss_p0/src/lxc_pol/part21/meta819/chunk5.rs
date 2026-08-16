//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3026/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3026(t15780: f64, t3302: f64, t1024: f64, t1043: f64, t1082: f64, t1089: f64, t11788: f64, t12073: f64, t12122: f64, t12127: f64, t12149: f64, t15604: f64, t15837: f64, t16432: f64, t16433: f64, t16436: f64, t16440: f64, t16449: f64, t16482: f64, t3059: f64, t3204: f64, t3278: f64, t354: f64, t43432: f64, t43453: f64, t43528: f64, t4772: f64, t4781: f64, t53273: f64, t54360: f64, t54931: f64, t54936: f64) -> (f64, f64) {
    let t55550 = t15780 * t3302;
    let t55562 = -0.39512695097613069591e1_f64 * t12122 * t16432 * t54931 + 0.19756347548806534796e1_f64 * t12127 * t16432 * t54936 - 0.65854491829355115987e0_f64 * t1024 * t1082 * t53273 - 0.79025390195226139182e1_f64 * t43432 * t16433 + 0.39512695097613069591e1_f64 * t12149 * t15837 * t1043 * t1089 + 0.39512695097613069591e1_f64 * t12149 * t4781 * t354 * t54360 + 0.19756347548806534796e1_f64 * t3278 * t16440 + 0.39512695097613069591e1_f64 * t43453 * t16436 + 0.39512695097613069591e1_f64 * t43528 * t16436 + 0.39512695097613069591e1_f64 * t12127 * t55550 * t15604 - 0.19756347548806534796e1_f64 * t1024 * t12073 * t4772 + 0.79025390195226139182e1_f64 * t11788 * t16482 + 0.39512695097613069591e1_f64 * t3204 * t16449 * t3059;
    (t55550, t55562)
}
