//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2804;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta740<F: Float>(t40628: F, t40834: F, t854: F, t10890: F, t2707: F, t10293: F, t240: F, t243: F, t813: F, t816: F, t10675: F, t2689: F, t798: F, t9726: F, t802: F, t10899: F, t794: F, t159: F, t216: F, t2475: F, t123: F, t212: F, t9291: F, t2786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40836, t40838, t40846, t40850, t40851) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2804::<F>(t40628, t40834, t854, t10890, t2707, t10293, t240, t243, t813, t816, t10675, t2689);
        let (t40861, t40862, t40864, t40868, t40921, t40922) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805::<F>(t798, t9726, t802, t10899, t794, t159, t216, t2475, t123, t212, t9291, t2786);
    (t40836, t40838, t40846, t40850, t40851, t40861, t40862, t40864, t40868, t40921, t40922)
}
