//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1332;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1333;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1334;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta209(t1120: f64, t5052: f64, t128: f64, t1121: f64, t4186: f64, t3357: f64, t3358: f64, t5044: f64, t5049: f64, t422: f64, t1130: f64, t1719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5053, t5054) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1332(t1120, t5052, t128);
        let t5056 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1333(t1121, t4186);
        let (t5057, t5058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1334(t1120, t5056, t128);
        let (t5060, t5062, t5063) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1335(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719);
    (t5053, t5054, t5056, t5057, t5058, t5060, t5062, t5063)
}
