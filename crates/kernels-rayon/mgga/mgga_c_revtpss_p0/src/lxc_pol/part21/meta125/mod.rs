//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk801;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk802;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk803;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk804;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk805;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta125(t2944: f64, t2970: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t324: f64, t960: f64, t964: f64, t320: f64, t963: f64, t315: f64, t972: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2971, t2974, t2979) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk801(t2944, t2970, t2846, t2848, t2855, t2860, t2864);
        let (t2980, t2982) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk802(t2979, t324, t960, t964);
        let (t2985, t2986) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk803(t320, t963);
        let t2987 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk804(t2986, t315);
        let t2988 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk805(t972);
        let t2989 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk806(t2988, t973);
    (t2971, t2974, t2979, t2980, t2982, t2985, t2986, t2987, t2988, t2989)
}
