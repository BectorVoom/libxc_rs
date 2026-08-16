//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk406;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk407;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk408;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk409;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk410;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk411;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta65<F: Float>(t1338: F, t236: F, t240: F, t1336: F, t241: F, t557: F, t67: F, t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t225: F, t680: F, t705: F, t68: F, t1307: F, t546: F, t548: F, t550: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t1339 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk406::<F>(t1338, t236);
        let (t1340, t1341) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk407::<F>(t1339, t240, t1336);
        let t1343 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk408::<F>(t241, t557, t67);
        let t1345 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk409::<F>(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t225, t680, t705);
        let (t1347, t1348, t1351) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk410::<F>(t557, t68, t1307, t1345, t546, t548);
        let t1352 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk411::<F>(t1351, t550);
        let t1354 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk412::<F>(t1343, t1352, t820);
    (t1339, t1340, t1341, t1343, t1345, t1347, t1348, t1351, t1352, t1354)
}
