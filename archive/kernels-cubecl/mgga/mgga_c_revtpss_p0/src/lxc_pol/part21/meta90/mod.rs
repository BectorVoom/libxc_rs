//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk627;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk628;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk629;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk630;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk631;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk632;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk633;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk634;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta90<F: Float>(t2236: F, t25: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F, t2233: F, t2235: F, t599: F, t602: F, t89: F, t90: F, t29: F, t644: F, t606: F, t70: F, t2: F, t580: F, t17: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2237 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk627::<F>(t2236);
        let (t2239, t2240, t2242) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk628::<F>(t2237, t25, t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk629::<F>(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk630::<F>(t2246, t29);
        let t2248 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk631::<F>(t644);
        let t2251 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk632::<F>(t606);
        let (t2252, t2255) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk633::<F>(t2251, t70, t2, t580);
        let t2256 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk634::<F>(t17, t2255);
        let t2257 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk635::<F>(t2256);
    (t2237, t2239, t2240, t2242, t2246, t2247, t2248, t2251, t2252, t2255, t2256, t2257)
}
