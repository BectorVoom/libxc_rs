//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk435;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk436;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk437;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk438;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk439;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta66<F: Float>(t1287: F, t17: F, t1284: F, t182: F, t521: F, t67: F, t758: F, t172: F, t763: F, t532: F, t571: F, t514: F, t25: F, t606: F, t517: F, zeta_threshold: F, t28: F, t1081: F, t215: F, t535: F, t782: F, t154: F, t547: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1288, t1290, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk435::<F>(t1287, t17, t1284, t182, t521, t67, t758, t172);
        let (t1296, t1297) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk436::<F>(t1294, t763, t532, t571);
        let t1298 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk437::<F>(t514);
        let (t1301, t1302) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk438::<F>(t25, t1298, t606, t517, zeta_threshold);
        let t1307 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk439::<F>(t28, t1081, t1302, t1301, zeta_threshold);
        let (t1313, t1314) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk440::<F>(t215, t535, t782, t154, t547);
    (t1288, t1290, t1291, t1293, t1294, t1296, t1297, t1298, t1302, t1307, t1313, t1314)
}
