//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta69 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk440;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk441;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk442;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk443;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk444;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk445;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk446;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk447;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta69(t1338: f64, t236: f64, t240: f64, t1336: f64, t241: f64, t557: f64, t67: f64, t1274: f64, t1276: f64, t1286: f64, t1288: f64, t1290: f64, t1293: f64, t1296: f64, t225: f64, t680: f64, t705: f64, t68: f64, t1307: f64, t546: f64, t548: f64, t550: f64, t820: f64, t248: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1339 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk440(t1338, t236);
        let t1340 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk441(t1339, t240);
        let t1341 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk442(t1336, t1340);
        let t1343 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk443(t241, t557, t67);
        let (t1345, t1347) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk444(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t225, t680, t705, t557, t68);
        let (t1348, t1351) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk445(t1307, t1347, t1345, t546, t548);
        let t1352 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk446(t1351, t550);
        let t1354 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk447(t1343, t1352, t820);
        let t1358 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk448(t248, t557, t836);
    (t1339, t1340, t1341, t1343, t1345, t1347, t1348, t1351, t1352, t1354, t1358)
}
