//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta72 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk480;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk481;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk482;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk483;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk484;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk485;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk486;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta72(t1378: f64, t1385: f64, t1324: f64, t1373: f64, t1375: f64, t568: f64, t570: f64, t1274: f64, t1276: f64, t1286: f64, t1288: f64, t1290: f64, t1293: f64, t1296: f64, t1297: f64, t1307: f64, t193: f64, t533: f64, t680: f64, t705: f64, t113: f64, t1266: f64, t1271: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t3: f64, t576: f64, t112: f64, t577: f64, t671: f64, t582: f64, t586: f64, t589: f64, t593: f64, t596: f64, t600: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1386 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk480(t1378, t1385);
        let t1388 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk481(t1324, t1373, t1375, t1386, t568);
        let t1390 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk482(t570);
        let t1393 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk483(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t1297, t1307, t1388, t1390, t193, t533, t680, t705);
        let t1395 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk484(t113, t1266, t1271, t1393, t510, t513, t574, t650, t652, t672);
        let (t1396, t1398, t1401) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk485(t1395, t3, t576, t112);
        let (t1404, t1406, t1408) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk486(t1395, t1401, t577, t671, t582, t586, t589, t593, t596, t600, t4, t581);
        let t1409 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk487(t25, t28, t1408, zeta_threshold);
    (t1386, t1388, t1390, t1393, t1395, t1396, t1398, t1401, t1404, t1406, t1408, t1409)
}
