//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk536;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk537;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta96<F: Float>(t2258: F, t36: F, t70: F, t607: F, t627: F, t362: F, t41: F, t47: F, t2251: F, t48: F, t59: F, t60: F, sigma0: F, t239: F, t64: F, t44: F, t49: F, t56: F, t614: F, t617: F, t38: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2259, t2260, t2263, t2270, t2275, t2276, t2279, t2282, t2283, t2286) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk536::<F>(t2258, t36, t70, t607, t627, t362, t41, t47, t2251, t48, t59, t60, sigma0);
        let t2289 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk537::<F>(t239, t64);
        let (t2290, t2291, t2292) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk538::<F>(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617, t38);
    (t2259, t2260, t2263, t2270, t2275, t2282, t2283, t2286, t2289, t2290, t2291, t2292)
}
