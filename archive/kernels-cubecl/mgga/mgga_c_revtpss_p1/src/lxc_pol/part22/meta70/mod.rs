//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk507;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk508;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk509;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk510;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk511;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk512;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk513;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk514;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk515;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta70<F: Float>(t1386: F, t239: F, t820: F, t240: F, t550: F, t72: F, t1319: F, t1322: F, t1332: F, t1334: F, t1336: F, t1339: F, t1342: F, t225: F, t679: F, t704: F, t73: F, t1353: F, t539: F, t541: F, t543: F, t828: F, t844: F, t247: F, t548: F, t235: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1388 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk507::<F>(t1386, t239, t820);
        let t1389 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk508::<F>(t240, t550);
        let t1390 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk509::<F>(t1389, t72);
        let t1392 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk510::<F>(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t225, t679, t704);
        let t1394 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk511::<F>(t550, t73);
        let t1395 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk512::<F>(t1353, t1394);
        let t1398 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk513::<F>(t1392, t1395, t539, t541);
        let t1399 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk514::<F>(t1398, t543);
        let (t1401, t1407, t1408) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk515::<F>(t1399, t828, t1390, t550, t844, t247, t548, t235, t545);
        let t1410 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk516::<F>(t1408, t239, t820);
    (t1388, t1389, t1390, t1392, t1394, t1395, t1398, t1399, t1401, t1407, t1408, t1410)
}
