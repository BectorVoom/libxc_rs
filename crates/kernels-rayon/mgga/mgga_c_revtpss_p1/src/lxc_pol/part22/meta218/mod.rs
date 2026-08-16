//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta218 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1377;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1378;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1379;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1380;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1381;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta218(t5296: f64, t5297: f64, t1042: f64, t3362: f64, t3617: f64, t4181: f64, t1012: f64, t1224: f64, t5052: f64, t3698: f64, t5047: f64, t482: f64, t5245: f64, t371: f64, t372: f64, t1234: f64, t1803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5298, t5299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1376(t5296, t5297, t1042);
        let t5302 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1377(t3362, t3617);
        let (t5303, t5304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1378(t4181, t5302, t1042);
        let t5308 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1379(t1012, t1224);
        let (t5309, t5312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1380(t5052, t5308, t1012, t3698);
        let (t5313, t5318, t5320) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1381(t5047, t5312, t482, t5245, t371, t372);
        let t5323 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1382(t1234, t1803);
    (t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312, t5313, t5318, t5320, t5323)
}
