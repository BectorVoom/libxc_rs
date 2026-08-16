//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1067 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1067(t46989: f64, t46993: f64, t22483: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t4135: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t5541: f64, t48255: f64, t46999: f64, t47005: f64, t47007: f64, t1448: f64, t5591: f64, t48260: f64, t48262: f64, t13648: f64, t13716: f64, t22496: f64, t39773: f64, t39783: f64, t4139: f64, t47003: f64, t5532: f64, t5542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73379, t73380, t73383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817(t46989, t46993, t22483, t39747, t39750, t39756, t39760, t4135, t46988, t46992, t46996, t46998, t5541);
        let (t73384, t73388, t73389, t73390, t73398, t73399, t73400) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818(t48255, t46999, t47005, t47007, t1448, t5591, t48260, t48262, t13648, t13716, t22496, t39773, t39783, t4139, t47003, t5532, t5542);
    (t73379, t73380, t73383, t73384, t73388, t73389, t73390, t73398, t73399, t73400)
}
