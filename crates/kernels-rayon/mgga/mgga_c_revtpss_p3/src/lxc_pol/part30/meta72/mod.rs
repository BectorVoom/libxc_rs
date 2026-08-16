//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta72 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk466;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk467;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk468;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk469;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk470;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk471;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk472;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta72(t550: f64, t844: f64, t247: f64, t548: f64, t235: f64, t545: f64, t239: f64, t820: f64, t530: f64, t549: f64, t240: f64, t72: f64, t1353: f64, t828: f64, t1368: f64, t1370: f64, t1372: f64, t1378: f64, t1383: f64, t1388: f64, t1401: f64, t225: f64, t561: f64, t213: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1405, t1407, t1408) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk466(t550, t844, t247, t548, t235, t545);
        let t1410 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk467(t1408, t239, t820);
        let t1412 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk468(t530, t549);
        let t1413 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk469(t1412, t240);
        let t1414 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk470(t1413, t72);
        let t1416 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk471(t1353, t1414, t828);
        let t1419 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk472(t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410, t1416);
        let (t1420, t1421, t1424) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk473(t1419, t225, t561, t213, t555);
    (t1405, t1407, t1408, t1410, t1412, t1413, t1414, t1416, t1419, t1420, t1421, t1424)
}
