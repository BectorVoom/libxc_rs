//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk626;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk627;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk628;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk629;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk630;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk631;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk632;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta91<F: Float>(t57: F, t635: F, t81: F, t116: F, t648: F, t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t99: F, t107: F, t200: F, t202: F, t205: F, t262: F, t705: F, t716: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2304, t2306) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk626::<F>(t57, t635, t81);
        let t2322 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk627::<F>(t116, t648);
        let (t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk628::<F>(t112, t2289, t625, t666, t111, t654);
        let t2349 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk629::<F>(t99);
        let t2357 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk630::<F>(t107);
        let t2375 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk631::<F>(t200);
        let t2382 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk632::<F>(t202);
        let (t2393, t2398) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk633::<F>(t205, t262, t705, t716);
    (t2304, t2306, t2322, t2335, t2336, t2339, t2349, t2357, t2375, t2382, t2393, t2398)
}
