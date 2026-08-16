//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta40 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk294;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk295;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk296;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk297;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk298;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta40<F: Float>(t159: F, t794: F, t222: F, t228: F, t216: F, t136: F, t220: F, t124: F, t775: F, t212: F, t27: F, t235: F, t240: F, t234: F, t243: F, t236: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t795, t797, t798, t799) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk294::<F>(t159, t794, t222, t228, t216);
        let t800 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk295::<F>(t136, t220);
        let (t802, t807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk296::<F>(t124, t775, t800, t212, t27);
        let t808 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk297::<F>(t235, t240);
        let t810 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk298::<F>(t234, t243, t808);
        let (t812, t813, t814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk299::<F>(t807, t810, t236, t786, t240, t27);
    (t795, t797, t798, t799, t800, t802, t807, t808, t810, t812, t813, t814)
}
