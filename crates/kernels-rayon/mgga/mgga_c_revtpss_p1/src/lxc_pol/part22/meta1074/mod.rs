//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1074 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3852;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3853;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1074(t39747: f64, t39750: f64, t39756: f64, t39760: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t73367: f64, t73371: f64, t73372: f64, t73373: f64, t73375: f64, t73379: f64, t73380: f64, t73384: f64, t73388: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t47003: f64, t47059: f64, t73389: f64, t73390: f64, t73398: f64, t73399: f64, t73402: f64, t73403: f64, t73411: f64, t73412: f64, t73416: f64, t73418: f64, t22212: f64, t2496: f64, t48280: f64, t48282: f64, t48285: f64, t1317: f64, t22193: f64, t39807: f64, t39813: f64, t47067: f64, t47070: f64, t47072: f64, t47076: f64, t73474: f64, t73477: f64, t73482: f64, t73494: f64, t73516: f64, t73517: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t74102 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3852(t39747, t39750, t39756, t39760, t46988, t46992, t46996, t46998, t73367, t73371, t73372, t73373, t73375, t73379, t73380, t73384, t73388);
        let t74103 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3853(t39773, t39783, t39786, t39791, t39795, t39799, t47003, t47059, t73389, t73390, t73398, t73399, t73402, t73403, t73411, t73412, t73416, t73418);
        let (t74107, t74108, t74109, t74110, t74112, t74113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3854(t22212, t2496, t48280, t48282, t48285, t1317, t22193, t39807, t39813, t47067, t47070, t47072, t47076, t73474, t73477, t73482, t73494, t73516, t73517);
    (t74102, t74103, t74107, t74108, t74109, t74110, t74112, t74113)
}
