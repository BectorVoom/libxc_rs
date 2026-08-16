//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk332;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk333;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk334;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk335;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta53<F: Float>(t1338: F, t236: F, t240: F, t1336: F, t241: F, t557: F, t67: F, t68: F, t248: F, t836: F, t555: F, t552: F, t531: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1339 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk332::<F>(t1338, t236);
        let (t1340, t1341) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk333::<F>(t1339, t240, t1336);
        let t1343 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk334::<F>(t241, t557, t67);
        let (t1347, t1358, t1360, t1361, t1362, t1363) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk335::<F>(t557, t68, t248, t836, t555, t236, t552, t240, t1336);
        let t1365 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk336::<F>(t531, t556);
    (t1339, t1340, t1341, t1343, t1347, t1358, t1360, t1361, t1362, t1363, t1365)
}
