//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2804;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta740(t40628: f64, t40834: f64, t854: f64, t10890: f64, t2707: f64, t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t10675: f64, t2689: f64, t798: f64, t9726: f64, t802: f64, t10899: f64, t794: f64, t159: f64, t216: f64, t2475: f64, t123: f64, t212: f64, t9291: f64, t2786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40836, t40838, t40846, t40850, t40851) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2804(t40628, t40834, t854, t10890, t2707, t10293, t240, t243, t813, t816, t10675, t2689);
        let (t40861, t40862, t40864, t40868, t40921, t40922) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805(t798, t9726, t802, t10899, t794, t159, t216, t2475, t123, t212, t9291, t2786);
    (t40836, t40838, t40846, t40850, t40851, t40861, t40862, t40864, t40868, t40921, t40922)
}
