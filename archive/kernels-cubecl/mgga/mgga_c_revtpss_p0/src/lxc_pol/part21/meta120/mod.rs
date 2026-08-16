//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk781;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk782;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk783;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta120<F: Float>(t2875: F, t935: F, t2874: F, t273: F, t276: F, t918: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t916: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2876, t2878, t2880) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk781::<F>(t2875, t935, t2874, t273, t276);
        let t2881 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk782::<F>(t918);
        let (t2882, t2884, t2889) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk783::<F>(t2880, t2881, t2846, t2848, t2855, t2860, t2864);
        let (t2890, t2892, t2897) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk784::<F>(t2889, t916, t2846, t273);
    (t2876, t2878, t2880, t2881, t2882, t2884, t2889, t2890, t2892, t2897)
}
