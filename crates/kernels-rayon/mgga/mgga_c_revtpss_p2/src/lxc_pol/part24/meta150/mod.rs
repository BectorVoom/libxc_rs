//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk763;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk764;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk765;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk766;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta150(t6096: f64, t904: f64, t128: f64, t5825: f64, t905: f64, t2847: f64, t4571: f64, t6094: f64, t291: f64, t1610: f64, t4590: f64, t1609: f64, t935: f64, t2874: f64, t1600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6097, t6098) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk763(t6096, t904, t128);
        let t6100 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk764(t5825, t905);
        let (t6101, t6102) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk765(t6100, t904, t128);
        let (t6104, t6106, t6108, t6109) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk766(t2847, t4571, t6094, t6098, t6102, t291, t1610, t4590, t1609);
        let (t6110, t6112, t6113) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk767(t6109, t935, t2874, t1600);
    (t6097, t6098, t6100, t6101, t6102, t6104, t6106, t6108, t6109, t6110, t6112, t6113)
}
