//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk460;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk461;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk462;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk463;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk464;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta73(t1353: f64, t1414: f64, t828: f64, t1368: f64, t1370: f64, t1372: f64, t1378: f64, t1383: f64, t1388: f64, t1401: f64, t1407: f64, t1410: f64, t225: f64, t561: f64, t213: f64, t555: f64, t560: f64, t545: f64, t869: f64, t689: f64, t546: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1416 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk460(t1353, t1414, t828);
        let t1419 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk461(t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410, t1416);
        let (t1420, t1421, t1424) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk462(t1419, t225, t561, t213, t555);
        let (t1425, t1426) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk463(t560);
        let t1427 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk464(t1426, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk465(t545, t555, t869, t689, t546, t786);
    (t1416, t1419, t1420, t1421, t1424, t1425, t1426, t1427, t1428, t1429, t1431, t1432)
}
