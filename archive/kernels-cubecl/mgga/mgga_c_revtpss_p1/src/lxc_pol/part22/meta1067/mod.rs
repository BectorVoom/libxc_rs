//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1067 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1067<F: Float>(t46989: F, t46993: F, t22483: F, t39747: F, t39750: F, t39756: F, t39760: F, t4135: F, t46988: F, t46992: F, t46996: F, t46998: F, t5541: F, t48255: F, t46999: F, t47005: F, t47007: F, t1448: F, t5591: F, t48260: F, t48262: F, t13648: F, t13716: F, t22496: F, t39773: F, t39783: F, t4139: F, t47003: F, t5532: F, t5542: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t73379, t73380, t73383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817::<F>(t46989, t46993, t22483, t39747, t39750, t39756, t39760, t4135, t46988, t46992, t46996, t46998, t5541);
        let (t73384, t73388, t73389, t73390, t73398, t73399, t73400) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818::<F>(t48255, t46999, t47005, t47007, t1448, t5591, t48260, t48262, t13648, t13716, t22496, t39773, t39783, t4139, t47003, t5532, t5542);
    (t73379, t73380, t73383, t73384, t73388, t73389, t73390, t73398, t73399, t73400)
}
