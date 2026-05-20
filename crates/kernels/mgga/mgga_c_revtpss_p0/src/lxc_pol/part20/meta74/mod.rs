//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk474;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta74<F: Float>(t47: F, t2251: F, t2258: F, t48: F, t59: F, t60: F, t239: F, t64: F, t2270: F, t44: F, t49: F, t56: F, t614: F, t617: F, t38: F) -> (F, F, F, F, F, F, F) {
        let (t2275, t2276, t2279, t2282, t2283, t2286, t2289) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk474::<F>(t47, t2251, t2258, t48, t59, t60, t239, t64);
        let (t2291, t2292) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk475::<F>(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617, t38);
    (t2275, t2276, t2279, t2282, t2289, t2291, t2292)
}
