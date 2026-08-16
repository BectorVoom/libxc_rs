//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta67 (260520-c91 hierarchical CSE).
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
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk481;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk482;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk483;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk484;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk485;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk486;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk487;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk488;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk489;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk490;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta67(t551: f64, t236: f64, t240: f64, t1336: f64, t241: f64, t557: f64, t67: f64, t1274: f64, t1276: f64, t1286: f64, t1288: f64, t1290: f64, t1293: f64, t1296: f64, t225: f64, t680: f64, t705: f64, t68: f64, t1307: f64, t546: f64, t548: f64, t550: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk481(t551);
        let t1339 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk482(t1338, t236);
        let t1340 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk483(t1339, t240);
        let t1341 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk484(t1336, t1340);
        let t1343 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk485(t241, t557, t67);
        let t1345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk486(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t225, t680, t705);
        let t1347 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk487(t557, t68);
        let t1348 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk488(t1307, t1347);
        let t1351 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk489(t1345, t1348, t546, t548);
        let t1352 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk490(t1351, t550);
        let t1354 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk491(t1343, t1352, t820);
    (t1337, t1338, t1339, t1340, t1341, t1343, t1345, t1347, t1348, t1351, t1352, t1354)
}
