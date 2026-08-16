//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta69 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk467;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk468;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk469;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk470;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk471;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk472;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk473;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk474;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk475;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta69<F: Float>(t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t225: F, t680: F, t705: F, t557: F, t68: F, t1307: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t248: F, t836: F, t555: F, t236: F, t552: F, t240: F, t1336: F, t531: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1345 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk467::<F>(t1274, t1276, t1286, t1288, t1290, t1293, t1296, t225, t680, t705);
        let t1347 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk468::<F>(t557, t68);
        let (t1348, t1351) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk469::<F>(t1307, t1347, t1345, t546, t548);
        let t1352 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk470::<F>(t1351, t550);
        let t1354 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk471::<F>(t1343, t1352, t820);
        let t1358 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk472::<F>(t248, t557, t836);
        let (t1360, t1361) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk473::<F>(t1358, t555, t236, t552);
        let t1362 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk474::<F>(t1361, t240);
        let t1363 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk475::<F>(t1336, t1362);
        let t1365 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk476::<F>(t531, t556);
    (t1345, t1347, t1348, t1351, t1352, t1354, t1358, t1360, t1361, t1362, t1363, t1365)
}
