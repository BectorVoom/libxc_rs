//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta118 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk802;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk803;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk804;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk805;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk806;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta118<F: Float>(t2858: F, t904: F, t128: F, t2258: F, t905: F, t2847: F, t2848: F, t2855: F, t291: F, t910: F, t914: F, t936: F, t287: F, t913: F, t275: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2859, t2860) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk802::<F>(t2858, t904, t128);
        let t2862 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk803::<F>(t2258, t905);
        let (t2863, t2864) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk804::<F>(t2862, t904, t128);
        let (t2866, t2868, t2869) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk805::<F>(t2847, t2848, t2855, t2860, t2864, t291, t910, t914);
        let (t2871, t2872, t2873) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk806::<F>(t2869, t936, t287, t913);
        let t2874 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk807::<F>(t275, t2873);
    (t2859, t2860, t2862, t2863, t2864, t2866, t2868, t2869, t2871, t2872, t2873, t2874)
}
