//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta71 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk458;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk459;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk460;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk461;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk462;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk463;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk464;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta71(t1377: f64, t68: f64, t1338: f64, t562: f64, t1352: f64, t1372: f64, t553: f64, t1332: f64, t1336: f64, t544: f64, t564: f64, t1324: f64, t1373: f64, t1375: f64, t568: f64, t570: f64, t1274: f64, t1276: f64, t1286: f64, t1288: f64, t1290: f64, t1293: f64, t1296: f64, t1297: f64, t1307: f64, t193: f64, t533: f64, t680: f64, t705: f64, t113: f64, t1266: f64, t1271: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1378 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk458(t1377, t68);
        let t1380 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk459(t1338, t562);
        let (t1381, t1383, t1385) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk460(t1352, t1380, t1372, t553, t1332, t1336, t544, t564);
        let t1386 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk461(t1378, t1385);
        let t1388 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk462(t1324, t1373, t1375, t1386, t568);
        let t1390 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk463(t570);
        let t1393 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk464(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t1297, t1307, t1388, t1390, t193, t533, t680, t705);
        let t1395 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk465(t113, t1266, t1271, t1393, t510, t513, t574, t650, t652, t672);
    (t1378, t1380, t1381, t1383, t1385, t1386, t1388, t1390, t1393, t1395)
}
