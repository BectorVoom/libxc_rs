//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1377;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1378;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1379;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1380;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1381;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta218<F: Float>(t5296: F, t5297: F, t1042: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F, t5052: F, t3698: F, t5047: F, t482: F, t5245: F, t371: F, t372: F, t1234: F, t1803: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5298, t5299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1376::<F>(t5296, t5297, t1042);
        let t5302 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1377::<F>(t3362, t3617);
        let (t5303, t5304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1378::<F>(t4181, t5302, t1042);
        let t5308 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1379::<F>(t1012, t1224);
        let (t5309, t5312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1380::<F>(t5052, t5308, t1012, t3698);
        let (t5313, t5318, t5320) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1381::<F>(t5047, t5312, t482, t5245, t371, t372);
        let t5323 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1382::<F>(t1234, t1803);
    (t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312, t5313, t5318, t5320, t5323)
}
