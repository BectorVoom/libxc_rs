//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk779;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk780;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk781;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta115<F: Float>(t2777: F, t870: F, t2439: F, t123: F, t212: F, t676: F, t225: F, t822: F, t251: F, t836: F, t231: F, t233: F, t860: F, t869: F, t689: F, t136: F, t2457: F, t2710: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk779::<F>(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk780::<F>(t225, t822);
        let t2786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk781::<F>(t251, t836, t231, t2783);
        let (t2787, t2789, t2790, t2791, t2793, t2796, t2797) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk782::<F>(t2782, t2786, t233, t860, t869, t689, t136, t251, t2457, t2710, t2783);
    (t2778, t2780, t2782, t2783, t2786, t2787, t2789, t2790, t2791, t2793, t2796, t2797)
}
