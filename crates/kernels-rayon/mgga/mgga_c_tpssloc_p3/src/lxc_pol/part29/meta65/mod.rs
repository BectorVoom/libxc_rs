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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk440;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk441;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk442;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk443;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk444;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk445;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk446;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta65(t1245: f64, t1246: f64, t1235: f64, t493: f64, t1201: f64, t1244: f64, t470: f64, t494: f64, t1241: f64, t1191: f64, t1236: f64, t1238: f64, t498: f64, t500: f64, t28: f64, t265: f64, t504: f64, t1096: f64, t1121: f64, t1161: f64, t1163: f64, t1168: f64, t193: f64, t336: f64, t873: f64, t1081: f64, t506: f64, t52: f64, t607: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1079: f64, t111: f64, t88: f64, t650: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1247, t1249, t1251) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk440(t1245, t1246, t1235, t493, t1201, t1244, t470, t494);
        let t1252 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk441(t1241, t1251);
        let t1254 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk442(t1191, t1236, t1238, t1252, t498);
        let t1256 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk443(t500);
        let (t1260, t1265) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk444(t28, t265, t504, t1096, t1121, t1161, t1163, t1168, t1254, t1256, t193, t336, t873, t1081, t506, t52, t607, dens_threshold, rho1, zeta_threshold);
        let t1266 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk445(t1079, t1265);
        let t1268 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk446(t111, t88);
        let t1271 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk447(t1268, t650, t671);
    (t1247, t1249, t1251, t1252, t1254, t1256, t1260, t1266, t1268, t1271)
}
