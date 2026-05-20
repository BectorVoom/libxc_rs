//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk462;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk463;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta75<F: Float>(t2231: F, t27: F, t592: F, t596: F, t21: F, t25: F, t89: F, t90: F, t29: F, t2: F, t580: F, t47: F, t59: F, t239: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk462::<F>(t2231, t27, t592, t596, t21);
        let t2237 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk463::<F>(t2236);
        let (t2239, t2246, t2247, t2255, t2275, t2282, t2289) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk464::<F>(t2237, t25, t89, t90, t29, t2, t580, t47, t59, t239, t64);
    (t2233, t2235, t2236, t2237, t2239, t2246, t2247, t2255, t2275, t2282, t2289)
}
