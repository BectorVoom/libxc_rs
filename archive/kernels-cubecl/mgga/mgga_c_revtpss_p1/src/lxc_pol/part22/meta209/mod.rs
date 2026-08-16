//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1332;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1333;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1334;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta209<F: Float>(t1120: F, t5052: F, t128: F, t1121: F, t4186: F, t3357: F, t3358: F, t5044: F, t5049: F, t422: F, t1130: F, t1719: F) -> (F, F, F, F, F, F, F, F) {
        let (t5053, t5054) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1332::<F>(t1120, t5052, t128);
        let t5056 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1333::<F>(t1121, t4186);
        let (t5057, t5058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1334::<F>(t1120, t5056, t128);
        let (t5060, t5062, t5063) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1335::<F>(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719);
    (t5053, t5054, t5056, t5057, t5058, t5060, t5062, t5063)
}
