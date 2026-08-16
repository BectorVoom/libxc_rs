//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3035/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3035(t1024: f64, t1083: f64, t11804: f64, t11940: f64, t12073: f64, t12143: f64, t12146: f64, t12163: f64, t15670: f64, t15717: f64, t15957: f64, t16390: f64, t16405: f64, t16502: f64, t1651: f64, t16515: f64, t16537: f64, t16540: f64, t3204: f64, t3278: f64, t3287: f64, t3291: f64, t43432: f64, t43450: f64, t43504: f64, t43528: f64, t4757: f64, t4977: f64, t5004: f64, t53865: f64) -> f64 {
    let t55854 = -0.19756347548806534796e1_f64 * t53865 * t1083 - 0.19756347548806534796e1_f64 * t3287 * t15957 * t16405 - 0.19756347548806534796e1_f64 * t43450 * t4977 + 0.39512695097613069591e1_f64 * t3204 * t12073 * t4757 - 0.19756347548806534796e1_f64 * t16502 * t12143 - 0.39512695097613069591e1_f64 * t43432 * t16537 + 0.39512695097613069591e1_f64 * t3204 * t5004 * t11804 - 0.65854491829355115987e0_f64 * t1024 * t43504 * t1651 + 0.39512695097613069591e1_f64 * t15670 * t12163 + 0.19756347548806534796e1_f64 * t3278 * t16515 - 0.11853808529283920877e2_f64 * t11940 * t3291 * t15717 + 0.19756347548806534796e1_f64 * t43528 * t16540 - 0.39512695097613069591e1_f64 * t12146 * t16390;
    t55854
}
