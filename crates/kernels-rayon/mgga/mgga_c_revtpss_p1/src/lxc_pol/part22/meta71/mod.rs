//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk517;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk518;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk519;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk520;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk521;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk522;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk523;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta71(t530: f64, t549: f64, t240: f64, t72: f64, t1353: f64, t828: f64, t1368: f64, t1370: f64, t1372: f64, t1378: f64, t1383: f64, t1388: f64, t1401: f64, t1407: f64, t1410: f64, t225: f64, t561: f64, t213: f64, t555: f64, t560: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1412 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk517(t530, t549);
        let t1413 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk518(t1412, t240);
        let t1414 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk519(t1413, t72);
        let (t1416, t1419) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk520(t1353, t1414, t828, t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410);
        let t1420 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk521(t1419, t225);
        let (t1421, t1424) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk522(t1420, t561, t213, t555);
        let (t1425, t1426) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk523(t560);
        let t1427 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk524(t1426, t225);
    (t1412, t1413, t1414, t1416, t1419, t1420, t1421, t1424, t1425, t1426, t1427)
}
