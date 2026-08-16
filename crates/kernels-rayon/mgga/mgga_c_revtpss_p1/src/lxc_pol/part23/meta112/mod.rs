//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk729;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk730;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk731;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk732;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta112(t240: f64, t68: f64, t281: f64, t283: f64, t698: f64, t931: f64, t1014: f64, t913: f64, t275: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2902, t2904, t2905, t2906, t2908) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk729(t240, t68, t281, t283, t698, t931, t1014);
        let t2922 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk730(t913);
        let t2923 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk731(t2922);
        let t2924 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk732(t275, t2923);
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk733(t290);
    (t2902, t2904, t2905, t2906, t2908, t2922, t2923, t2924, t2925, t2926)
}
