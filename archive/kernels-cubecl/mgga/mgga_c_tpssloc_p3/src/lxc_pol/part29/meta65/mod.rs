//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta65 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk440;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk441;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk442;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk443;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk444;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk445;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk446;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta65<F: Float>(t1245: F, t1246: F, t1235: F, t493: F, t1201: F, t1244: F, t470: F, t494: F, t1241: F, t1191: F, t1236: F, t1238: F, t498: F, t500: F, t28: F, t265: F, t504: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t193: F, t336: F, t873: F, t1081: F, t506: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1079: F, t111: F, t88: F, t650: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1247, t1249, t1251) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk440::<F>(t1245, t1246, t1235, t493, t1201, t1244, t470, t494);
        let t1252 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk441::<F>(t1241, t1251);
        let t1254 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk442::<F>(t1191, t1236, t1238, t1252, t498);
        let t1256 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk443::<F>(t500);
        let (t1260, t1265) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk444::<F>(t28, t265, t504, t1096, t1121, t1161, t1163, t1168, t1254, t1256, t193, t336, t873, t1081, t506, t52, t607, dens_threshold, rho1, zeta_threshold);
        let t1266 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk445::<F>(t1079, t1265);
        let t1268 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk446::<F>(t111, t88);
        let t1271 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk447::<F>(t1268, t650, t671);
    (t1247, t1249, t1251, t1252, t1254, t1256, t1260, t1266, t1268, t1271)
}
