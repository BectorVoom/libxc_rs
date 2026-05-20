//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta49 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk333;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk334;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk335;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk336;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk337;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk338;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta49<F: Float>(t340: F, t338: F, t378: F, t225: F, t385: F, t902: F, t908: F, t344: F, t614: F, t139: F, t221: F, t346: F, t345: F, t220: F, t44: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t992, t993, t994) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk333::<F>(t340, t338);
        let t995 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk334::<F>(t378, t994);
        let t996 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk335::<F>(t225, t385);
        let t999 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk336::<F>(t902, t908);
        let t1000 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk337::<F>(t996, t999);
        let (t1003, t1007, t1009, t1010) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk338::<F>(t344, t614, t139, t221, t346, t345, t220);
        let t1011 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk339::<F>(t1010, t44);
    (t992, t993, t994, t995, t996, t999, t1000, t1003, t1007, t1009, t1010, t1011)
}
