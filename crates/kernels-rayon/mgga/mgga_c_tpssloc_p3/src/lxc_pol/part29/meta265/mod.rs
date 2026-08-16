//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1245;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1246;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1247;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1248;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1249;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1250;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta265(t1241: f64, t7391: f64, t1238: f64, t1252: f64, t2121: f64, t2155: f64, t3487: f64, t3593: f64, t498: f64, t7282: f64, t7283: f64, t7288: f64, t7291: f64, t7296: f64, t7303: f64, t7306: f64, t7349: f64, t7351: f64, t7356: f64, t2157: f64, t3640: f64, t28: f64, t265: f64, t504: f64, t1254: f64, t1256: f64, t193: f64, t336: f64, t4700: f64, t6834: f64, t2161: f64, t52: f64, t607: f64, t6855: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t7279: f64, t671: f64, t6867: f64, t6869: f64, t6871: f64, t7264: f64, t7266: f64, t113: f64, t1266: f64, t1393: f64, t2114: f64, t2165: f64, t2167: f64, t510: f64, t574: f64, t650: f64, t652: f64, t6522: f64, t6524: f64, t6527: f64, t6537: f64, t672: f64, t6877: f64, t6882: f64, t6998: f64, t7001: f64, t7271: f64, t3: f64, t112: f64, t2169: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7392 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1245(t1241, t7391);
        let t7394 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1246(t1238, t1252, t2121, t2155, t3487, t3593, t498, t7282, t7283, t7288, t7291, t7296, t7303, t7306, t7349, t7351, t7356, t7392);
        let t7398 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1247(t2157, t3640);
        let (t7402, t7407) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1248(t28, t265, t504, t1254, t1256, t193, t336, t4700, t6834, t7394, t7398, t2161, t52, t607, t6855, dens_threshold, rho1, zeta_threshold);
        let t7408 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1249(t7279, t7407);
        let (t7412, t7415) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1250(t671, t6867, t6869, t6871, t7264, t7266, t113, t1266, t1393, t2114, t2165, t2167, t510, t574, t650, t652, t6522, t6524, t6527, t6537, t672, t6877, t6882, t6998, t7001, t7271, t7408);
        let (t7416, t7423) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1251(t3, t7415, t112, t2169);
    (t7392, t7394, t7398, t7402, t7408, t7412, t7415, t7416, t7423)
}
