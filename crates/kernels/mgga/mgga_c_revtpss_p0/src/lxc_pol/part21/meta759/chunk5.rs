//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2683/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683<F: Float>(t1398: F, t2782: F, t4086: F, t543: F, t5710: F, t13805: F, t1399: F, t14122: F, t14127: F, t14193: F, t3924: F, t4004: F, t48015: F, t49313: F, t49322: F, t49325: F, t49327: F, t5745: F, t5755: F, t820: F, t9995: F) -> F {
    let t49346 = t2782 * t4086 * t5710 * t1398 * t543;
    let t49348 = F::cast_from(0.16463622957338778996e-1_f64) * t49313 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t14127 * t4004 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t14127 * t13805 + t49322 - F::cast_from(0.58544643236296698113e-1_f64) * t49325 - F::cast_from(0.39512695097613069591e1_f64) * t820 * t49327 * t9995 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t48015 * t1399 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t14122 * t13805 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t14122 * t3924 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t14127 * t3924 + F::cast_from(0.32927245914677557992e-1_f64) * t49346;
    t49348
}
