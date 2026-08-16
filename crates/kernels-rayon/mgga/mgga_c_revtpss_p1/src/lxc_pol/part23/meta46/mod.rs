//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk336;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk337;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk338;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta46(t916: f64, t918: f64, t902: f64, t273: f64, t240: f64, t696: f64, t281: f64, t283: f64, t346: f64, t906: f64, t141: f64, t908: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t919, t921, t923) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk336(t916, t918, t902, t273);
        let (t924, t926, t928, t929, t930) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk337(t918, t923, t240, t696, t281, t283, t346);
        let (t931, t932, t934) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk338(t906, t930, t141, t908, t919, t921, t924, t929);
        let t935 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk339(t290);
    (t919, t921, t923, t924, t926, t928, t929, t930, t931, t932, t934, t935)
}
