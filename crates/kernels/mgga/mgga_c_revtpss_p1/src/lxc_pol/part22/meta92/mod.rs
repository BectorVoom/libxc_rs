//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk657;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk658;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk659;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk660;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta92<F: Float>(t30: F, t33: F, t2257: F, zeta_threshold: F, t36: F, t70: F, t607: F, t627: F, t362: F, t41: F, sigma0: F, t47: F, t2251: F, t48: F, t59: F, t60: F, t239: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2258 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk657::<F>(t30, t33, t2257, zeta_threshold);
        let (t2259, t2260, t2263, t2270) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk658::<F>(t2258, t36, t70, t607, t627, t362, t41, sigma0);
        let t2275 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk659::<F>(t47);
        let (t2276, t2279, t2282) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk660::<F>(t2251, t2275, t2258, t48, t59);
        let (t2283, t2286, t2289) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk661::<F>(t2251, t2282, t2258, t60, t239, t64);
    (t2258, t2259, t2260, t2263, t2270, t2275, t2276, t2279, t2282, t2283, t2286, t2289)
}
