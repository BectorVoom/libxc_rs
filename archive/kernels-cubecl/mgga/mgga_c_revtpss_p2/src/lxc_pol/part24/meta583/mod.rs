//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta583<F: Float>(t73350: F, t48225: F, t85895: F, t48227: F, t73360: F, t48243: F, t39483: F, t39520: F, t39528: F, t39531: F, t39747: F, t46972: F, t46980: F, t48262: F, t39750: F, t39756: F, t39760: F, t39773: F, t39783: F, t46988: F, t46992: F, t46996: F, t46998: F, t47000: F, t47003: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91958, t91959, t91960, t91961, t91962, t91963, t91964) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814::<F>(t73350, t48225, t85895, t48227, t73360, t48243, t39483, t39520, t39528, t39531, t39747, t46972, t46980);
        let (t91966, t91967) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815::<F>(t48262, t39750, t39756, t39760, t39773, t39783, t46988, t46992, t46996, t46998, t47000, t47003);
    (t91958, t91959, t91960, t91961, t91962, t91963, t91964, t91966, t91967)
}
