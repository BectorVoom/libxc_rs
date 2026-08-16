//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta583(t73350: f64, t48225: f64, t85895: f64, t48227: f64, t73360: f64, t48243: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39747: f64, t46972: f64, t46980: f64, t48262: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t39783: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47000: f64, t47003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91958, t91959, t91960, t91961, t91962, t91963, t91964) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814(t73350, t48225, t85895, t48227, t73360, t48243, t39483, t39520, t39528, t39531, t39747, t46972, t46980);
        let (t91966, t91967) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815(t48262, t39750, t39756, t39760, t39773, t39783, t46988, t46992, t46996, t46998, t47000, t47003);
    (t91958, t91959, t91960, t91961, t91962, t91963, t91964, t91966, t91967)
}
