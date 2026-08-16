//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk471;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk472;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk473;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk474;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk475;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk476;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk477;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk478;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk479;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta65<F: Float>(t1269: F, t225: F, t494: F, t460: F, t487: F, t493: F, t473: F, t1214: F, t1032: F, t1243: F, t355: F, t471: F, t1248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1270, t1271) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk471::<F>(t1269, t225, t494);
        let t1274 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk472::<F>(t460, t487);
        let (t1275, t1276) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk473::<F>(t493);
        let t1277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk474::<F>(t1276, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk475::<F>(t473, t487);
        let t1281 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk476::<F>(t1214, t1280);
        let t1284 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk477::<F>(t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk478::<F>(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk479::<F>(t355, t471);
        let t1288 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk480::<F>(t1248, t1287, t487);
    (t1270, t1271, t1274, t1275, t1276, t1277, t1280, t1281, t1284, t1285, t1287, t1288)
}
