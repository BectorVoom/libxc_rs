//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta965 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta965<F: Float>(t73352: F, t177: F, t22789: F, t762: F, t48227: F, t46973: F, t48243: F, t46977: F, t39483: F, t39520: F, t39528: F, t39531: F, t48224: F, t48226: F, t48234: F, t48236: F, t48241: F, t48244: F, t48248: F, t73374: F, t46989: F, t46993: F, t47005: F, t39747: F, t39750: F, t39756: F, t39760: F, t39773: F, t46988: F, t46992: F, t46996: F, t46998: F, t47003: F, t48256: F, t48259: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85893, t85896, t85897, t85898, t85899, t85900, t85901) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262::<F>(t73352, t177, t22789, t762, t48227, t46973, t48243, t46977, t39483, t39520, t39528, t39531, t48224, t48226, t48234, t48236, t48241, t48244, t48248);
        let (t85903, t85904, t85905, t85906, t85907) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263::<F>(t73374, t46989, t46993, t47005, t39747, t39750, t39756, t39760, t39773, t46988, t46992, t46996, t46998, t47003, t48256, t48259);
    (t85893, t85896, t85897, t85898, t85899, t85900, t85901, t85903, t85904, t85905, t85906, t85907)
}
