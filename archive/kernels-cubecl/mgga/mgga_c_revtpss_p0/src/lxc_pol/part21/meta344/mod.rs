//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1674;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta344<F: Float>(t11591: F, t983: F, t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11392: F, t11394: F, t11398: F, t11590: F, t3022: F, t3026: F, t11467: F, t3011: F, t973: F, t981: F, t2986: F, t972: F, t3007: F, t11465: F, t3014: F, t11501: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11593, t11594) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1674::<F>(t11591, t983, t11291, t11293, t11296, t11303, t11382, t11390, t11392, t11394, t11398, t11590);
        let (t11596, t11598, t11600, t11602, t11604, t11606, t11608, t11610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1675::<F>(t3022, t3026, t11467, t3011, t973, t981, t2986, t972, t3007, t11465, t3014, t11501, t964);
    (t11593, t11594, t11596, t11598, t11600, t11602, t11604, t11606, t11608, t11610)
}
