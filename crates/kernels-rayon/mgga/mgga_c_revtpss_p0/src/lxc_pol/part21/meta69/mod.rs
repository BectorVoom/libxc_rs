//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta69 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk505;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk506;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk507;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk508;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk509;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk510;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk511;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk512;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk513;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta69(t1385: f64, t235: f64, t239: f64, t820: f64, t240: f64, t550: f64, t72: f64, t1319: f64, t1322: f64, t1332: f64, t1334: f64, t1336: f64, t1339: f64, t1342: f64, t225: f64, t679: f64, t704: f64, t73: f64, t1353: f64, t539: f64, t541: f64, t543: f64, t828: f64, t844: f64, t247: f64, t548: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1386 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk505(t1385, t235);
        let t1388 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk506(t1386, t239, t820);
        let t1389 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk507(t240, t550);
        let t1390 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk508(t1389, t72);
        let t1392 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk509(t1319, t1322, t1332, t1334, t1336, t1339, t1342, t225, t679, t704);
        let t1394 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk510(t550, t73);
        let t1395 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk511(t1353, t1394);
        let t1398 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk512(t1392, t1395, t539, t541);
        let t1399 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk513(t1398, t543);
        let (t1401, t1407, t1408) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk514(t1399, t828, t1390, t550, t844, t247, t548, t235, t545);
    (t1386, t1388, t1389, t1390, t1392, t1394, t1395, t1398, t1399, t1401, t1407, t1408)
}
