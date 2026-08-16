//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk932;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk933;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta186<F: Float>(t3995: F, t9775: F, t1408: F, t2681: F, t820: F, t1416: F, t124: F, t212: F, t2237: F, t800: F, t1376: F, t123: F, t125: F, t2452: F, t9720: F, t235: F, t4086: F, t2453: F, t240: F, t2712: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9776, t9779, t9780, t9784, t9786, t9789) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk932::<F>(t3995, t9775, t1408, t2681, t820, t1416, t124, t212, t2237, t800, t1376, t123, t125, t2452, t9720);
        let (t9791, t9792, t9793) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk933::<F>(t1376, t9789, t235, t4086, t2453);
        let t9794 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk934::<F>(t240, t2712);
    (t9776, t9779, t9780, t9784, t9786, t9789, t9791, t9792, t9793, t9794)
}
