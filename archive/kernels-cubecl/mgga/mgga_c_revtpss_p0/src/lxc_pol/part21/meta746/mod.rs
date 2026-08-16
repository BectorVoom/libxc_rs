//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta746 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta746<F: Float>(t46977: F, t46979: F, t46981: F, t46983: F, t46989: F, t46993: F, t187: F, t48216: F, t13597: F, t2516: F, t39747: F, t39750: F, t39756: F, t39760: F, t46988: F, t46992: F, t46996: F, t46998: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48247, t48248, t48249, t48250, t48251, t48252, t48254, t48256, t48257) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2620::<F>(t46977, t46979, t46981, t46983, t46989, t46993, t187, t48216, t13597, t2516, t39747, t39750, t39756, t39760, t46988, t46992, t46996, t46998);
    (t48247, t48248, t48249, t48250, t48251, t48252, t48254, t48256, t48257)
}
