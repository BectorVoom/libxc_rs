//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk448;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk449;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk450;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta66<F: Float>(t25: F, t28: F, t522: F, t588: F, t592: F, t514: F, t606: F, t1081: F, t517: F, t157: F, zeta_threshold: F, t184: F, t17: F, t521: F, t750: F, t182: F, t67: F, t758: F, t172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1274, t1276, t1284) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk448::<F>(t25, t28, t522, t588, t592, t514, t606, t1081, t517, t157, zeta_threshold);
        let t1285 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk449::<F>(t1284, t184);
        let (t1286, t1287) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk450::<F>(t1285, t17, t521, t750);
        let (t1288, t1290, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk451::<F>(t1287, t17, t1284, t182, t521, t67, t758, t172);
    (t1274, t1276, t1284, t1285, t1286, t1287, t1288, t1290, t1291, t1293, t1294)
}
