//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk751;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk752;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk753;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk754;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta110<F: Float>(t221: F, t2675: F, t775: F, t2674: F, t26: F, t66: F, t240: F, t243: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F, t810: F, t854: F, t236: F, t807: F, t21: F, t65: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2677, t2678, t2681) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk751::<F>(t221, t2675, t775, t2674, t26, t66);
        let t2682 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk752::<F>(t240, t2681);
        let (t2686, t2689) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk753::<F>(t243, t247, t2682, t237, t124, t212, t596, t800);
        let (t2691, t2693, t2694, t2695, t2698) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk754::<F>(t2689, t810, t775, t854, t236, t807, t21, t65);
        let t2699 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk755::<F>(t2698, t64);
    (t2677, t2678, t2681, t2682, t2686, t2689, t2691, t2693, t2694, t2695, t2698, t2699)
}
