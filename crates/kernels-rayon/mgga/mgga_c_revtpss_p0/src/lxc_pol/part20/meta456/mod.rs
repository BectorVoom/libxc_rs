//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1740;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta456(t1340: f64, t40086: f64, t4038: f64, t9318: f64, t1337: f64, t40101: f64, t9323: f64, t40097: f64, t39816: f64, t1333: f64, t9855: f64, t19: f64, t2237: f64, t521: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1740(t1340, t40086, t4038, t9318, t1337, t40101, t9323, t40097, t39816, t1333, t9855, t19, t2237, t521);
        let t47004 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1741(t39747, t39750, t39756, t39760, t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003);
    (t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003, t47004)
}
