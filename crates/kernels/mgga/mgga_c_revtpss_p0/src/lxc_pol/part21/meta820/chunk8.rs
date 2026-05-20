//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3035/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3035<F: Float>(t1024: F, t1083: F, t11804: F, t11940: F, t12073: F, t12143: F, t12146: F, t12163: F, t15670: F, t15717: F, t15957: F, t16390: F, t16405: F, t16502: F, t1651: F, t16515: F, t16537: F, t16540: F, t3204: F, t3278: F, t3287: F, t3291: F, t43432: F, t43450: F, t43504: F, t43528: F, t4757: F, t4977: F, t5004: F, t53865: F) -> F {
    let t55854 = -F::cast_from(0.19756347548806534796e1_f64) * t53865 * t1083 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t15957 * t16405 - F::cast_from(0.19756347548806534796e1_f64) * t43450 * t4977 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t12073 * t4757 - F::cast_from(0.19756347548806534796e1_f64) * t16502 * t12143 - F::cast_from(0.39512695097613069591e1_f64) * t43432 * t16537 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t5004 * t11804 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t43504 * t1651 + F::cast_from(0.39512695097613069591e1_f64) * t15670 * t12163 + F::cast_from(0.19756347548806534796e1_f64) * t3278 * t16515 - F::cast_from(0.11853808529283920877e2_f64) * t11940 * t3291 * t15717 + F::cast_from(0.19756347548806534796e1_f64) * t43528 * t16540 - F::cast_from(0.39512695097613069591e1_f64) * t12146 * t16390;
    t55854
}
