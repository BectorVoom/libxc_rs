//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk640;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk641;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk642;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk643;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta92<F: Float>(t2251: F, t2282: F, t2258: F, t60: F, t239: F, t64: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F, t38: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t633: F, t637: F, t77: F, t2252: F, t2260: F, t2263: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2283, t2286, t2289) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk640::<F>(t2251, t2282, t2258, t60, t239, t64);
        let (t2290, t2291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk641::<F>(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617);
        let (t2292, t2297, t2299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk642::<F>(t2291, t38, t45, t631, t78);
        let (t2304, t2306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk643::<F>(t57, t635, t81);
        let t2312 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644::<F>(t2251, t2258, t2299, t2306, t633, t637, t77);
        let t2315 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk645::<F>(t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
    (t2283, t2286, t2289, t2290, t2291, t2292, t2297, t2299, t2304, t2306, t2312, t2315)
}
