//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk552;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk553;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk554;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk555;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk556;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk557;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta96(t1087: f64, t3090: f64, t1066: f64, t828: f64, t357: f64, t905: f64, t1065: f64, t126: f64, t1086: f64, t994: f64, t373: f64, t66: f64, t1024: f64, t1062: f64, t1031: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3091 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk552(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk553(t1066, t828);
        let (t3094, t3109) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk554(t357, t905, t1065, t126);
        let (t3114, t3115) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk555(t1086, t994, t3090);
        let (t3116, t3117) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk556(t373, t66, t828);
        let t3127 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk557(t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk558(t1031, t196);
    (t3091, t3092, t3094, t3109, t3114, t3115, t3116, t3117, t3127, t3140)
}
