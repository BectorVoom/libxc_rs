//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta125 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk801;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk802;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk803;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk804;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk805;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta125<F: Float>(t2944: F, t2970: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t324: F, t960: F, t964: F, t320: F, t963: F, t315: F, t972: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2971, t2974, t2979) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk801::<F>(t2944, t2970, t2846, t2848, t2855, t2860, t2864);
        let (t2980, t2982) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk802::<F>(t2979, t324, t960, t964);
        let (t2985, t2986) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk803::<F>(t320, t963);
        let t2987 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk804::<F>(t2986, t315);
        let t2988 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk805::<F>(t972);
        let t2989 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk806::<F>(t2988, t973);
    (t2971, t2974, t2979, t2980, t2982, t2985, t2986, t2987, t2988, t2989)
}
