//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 934/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk934(t1255: f64, t980: f64, t1252: f64, t3646: f64, t457: f64, t13485: f64, t13487: f64, t452: f64, t1004: f64, t3829: f64, t1035: f64, t14255: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14460 = t980 * t1255;
    let t14478 = t980 * t1252;
    let t14480 = t3646 * t457;
    let t14485 = 0.15805078039045227836e2_f64 * t13485 * t452 * t13487;
    let t14486 = t1004 * t3829;
    let t14490 = 0.39512695097613069591e1_f64 * t1035 * t452 * t14255;
    (t14460, t14478, t14480, t14485, t14486, t14490)
}
