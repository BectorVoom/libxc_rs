//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk427;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk428;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk429;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk430;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk431;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk432;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk433;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta65<F: Float>(t1191: F, t1236: F, t1238: F, t1252: F, t498: F, t500: F, t28: F, t265: F, t504: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t193: F, t336: F, t873: F, t1081: F, t506: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1079: F, t111: F, t88: F, t650: F, t671: F, t25: F, t522: F, t588: F, t592: F, t514: F, t606: F, t517: F, t157: F, t184: F, t17: F, t521: F, t750: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1254, t1256) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk427::<F>(t1191, t1236, t1238, t1252, t498, t500);
        let (t1260, t1265) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk428::<F>(t28, t265, t504, t1096, t1121, t1161, t1163, t1168, t1254, t1256, t193, t336, t873, t1081, t506, t52, t607, dens_threshold, rho1, zeta_threshold);
        let t1266 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk429::<F>(t1079, t1265);
        let t1268 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk430::<F>(t111, t88);
        let t1271 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk431::<F>(t1268, t650, t671);
        let (t1274, t1276, t1284) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk432::<F>(t25, t28, t522, t588, t592, t514, t606, t1081, t517, t157, zeta_threshold);
        let t1285 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk433::<F>(t1284, t184);
        let (t1286, t1287) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk434::<F>(t1285, t17, t521, t750);
    (t1254, t1256, t1260, t1266, t1268, t1271, t1274, t1276, t1284, t1285, t1286, t1287)
}
