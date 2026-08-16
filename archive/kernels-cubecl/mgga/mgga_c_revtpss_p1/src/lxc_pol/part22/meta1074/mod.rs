//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1074 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3852;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3853;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1074<F: Float>(t39747: F, t39750: F, t39756: F, t39760: F, t46988: F, t46992: F, t46996: F, t46998: F, t73367: F, t73371: F, t73372: F, t73373: F, t73375: F, t73379: F, t73380: F, t73384: F, t73388: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t47003: F, t47059: F, t73389: F, t73390: F, t73398: F, t73399: F, t73402: F, t73403: F, t73411: F, t73412: F, t73416: F, t73418: F, t22212: F, t2496: F, t48280: F, t48282: F, t48285: F, t1317: F, t22193: F, t39807: F, t39813: F, t47067: F, t47070: F, t47072: F, t47076: F, t73474: F, t73477: F, t73482: F, t73494: F, t73516: F, t73517: F) -> (F, F, F, F, F, F, F, F) {
        let t74102 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3852::<F>(t39747, t39750, t39756, t39760, t46988, t46992, t46996, t46998, t73367, t73371, t73372, t73373, t73375, t73379, t73380, t73384, t73388);
        let t74103 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3853::<F>(t39773, t39783, t39786, t39791, t39795, t39799, t47003, t47059, t73389, t73390, t73398, t73399, t73402, t73403, t73411, t73412, t73416, t73418);
        let (t74107, t74108, t74109, t74110, t74112, t74113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3854::<F>(t22212, t2496, t48280, t48282, t48285, t1317, t22193, t39807, t39813, t47067, t47070, t47072, t47076, t73474, t73477, t73482, t73494, t73516, t73517);
    (t74102, t74103, t74107, t74108, t74109, t74110, t74112, t74113)
}
