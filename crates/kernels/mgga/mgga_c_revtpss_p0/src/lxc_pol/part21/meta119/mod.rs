//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta119 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk775;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk776;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk777;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk778;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk779;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta119<F: Float>(t2258: F, t905: F, t904: F, t128: F, t2847: F, t2848: F, t2855: F, t2860: F, t291: F, t910: F, t914: F, t936: F, t287: F, t913: F, t275: F, t934: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2862 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk775::<F>(t2258, t905);
        let (t2863, t2864) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk776::<F>(t2862, t904, t128);
        let (t2866, t2868, t2869) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk777::<F>(t2847, t2848, t2855, t2860, t2864, t291, t910, t914);
        let (t2871, t2872, t2873) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk778::<F>(t2869, t936, t287, t913);
        let t2874 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk779::<F>(t275, t2873);
        let t2875 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk780::<F>(t934);
    (t2862, t2863, t2864, t2866, t2868, t2869, t2871, t2872, t2873, t2874, t2875)
}
