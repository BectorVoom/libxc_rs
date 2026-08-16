//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2683/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683(t1398: f64, t2782: f64, t4086: f64, t543: f64, t5710: f64, t13805: f64, t1399: f64, t14122: f64, t14127: f64, t14193: f64, t3924: f64, t4004: f64, t48015: f64, t49313: f64, t49322: f64, t49325: f64, t49327: f64, t5745: f64, t5755: f64, t820: f64, t9995: f64) -> f64 {
    let t49346 = t2782 * t4086 * t5710 * t1398 * t543;
    let t49348 = 0.16463622957338778996e-1_f64 * t49313 + 0.11853808529283920877e2_f64 * t5745 * t14127 * t4004 - 0.11853808529283920877e2_f64 * t14193 * t14127 * t13805 + t49322 - 0.58544643236296698113e-1_f64 * t49325 - 0.39512695097613069591e1_f64 * t820 * t49327 * t9995 - 0.19756347548806534796e1_f64 * t5755 * t48015 * t1399 - 0.11853808529283920877e2_f64 * t14193 * t14122 * t13805 - 0.19756347548806534796e1_f64 * t5755 * t14122 * t3924 - 0.19756347548806534796e1_f64 * t5755 * t14127 * t3924 + 0.32927245914677557992e-1_f64 * t49346;
    t49348
}
