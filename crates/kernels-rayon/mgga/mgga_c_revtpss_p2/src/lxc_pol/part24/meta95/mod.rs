//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk545;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk546;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk547;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk548;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk549;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk550;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta95(t3011: f64, t315: f64, t323: f64, t2846: f64, t340: f64, t992: f64, t338: f64, t378: f64, t221: f64, t346: f64, t696: f64, t345: f64, t360: f64, t365: f64, t1038: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3012 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk545(t3011, t315);
        let (t3013, t3014) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk546(t323);
        let (t3037, t3056, t3057) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk547(t2846, t340, t992, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk548(t3057, t378);
        let (t3070, t3082, t3088) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk549(t2846, t221, t346, t696, t345, t360, t365);
        let t3089 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk550(t1038, t72);
        let t3090 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk551(t3088, t3089);
    (t3012, t3013, t3014, t3037, t3056, t3057, t3058, t3070, t3082, t3088, t3089, t3090)
}
