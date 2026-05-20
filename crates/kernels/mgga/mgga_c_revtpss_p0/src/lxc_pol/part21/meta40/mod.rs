//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta40 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk297;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk298;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk299;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk300;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk301;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk302;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta40<F: Float>(t124: F, t775: F, t800: F, t212: F, t27: F, t235: F, t240: F, t234: F, t243: F, t236: F, t786: F, t213: F, t225: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t802, t807) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk297::<F>(t124, t775, t800, t212, t27);
        let t808 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk298::<F>(t235, t240);
        let t810 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk299::<F>(t234, t243, t808);
        let (t812, t813, t814) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk300::<F>(t807, t810, t236, t786, t240, t27);
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk301::<F>(t243, t814, t124, t800);
        let (t819, t820) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk302::<F>(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk303::<F>(t232);
    (t802, t807, t808, t810, t812, t813, t814, t816, t819, t820, t821, t822)
}
