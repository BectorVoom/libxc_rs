//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta269 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1225;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1226;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1227;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1228;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1229;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1230;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta269(t671: f64, t6867: f64, t6869: f64, t6871: f64, t7264: f64, t7266: f64, t113: f64, t1266: f64, t1393: f64, t2114: f64, t2165: f64, t2167: f64, t510: f64, t574: f64, t650: f64, t652: f64, t6522: f64, t6524: f64, t6527: f64, t6537: f64, t672: f64, t6877: f64, t6882: f64, t6998: f64, t7001: f64, t7271: f64, t7408: f64, t3: f64, t112: f64, t2169: f64, t577: f64, t7014: f64, t7017: f64, t7019: f64, t33: f64, t3953: f64, t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1433: f64, t71: f64, t1874: f64, t4028: f64, t1458: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7412, t7415) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1225(t671, t6867, t6869, t6871, t7264, t7266, t113, t1266, t1393, t2114, t2165, t2167, t510, t574, t650, t652, t6522, t6524, t6527, t6537, t672, t6877, t6882, t6998, t7001, t7271, t7408);
        let (t7416, t7423) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1226(t3, t7415, t112, t2169);
        let (t7426, t7428) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1227(t577, t671, t7014, t7017, t7019, t7415, t7423, t33, t3953);
        let (t7431, t7432) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1228(t1437, t79, t72);
        let t7435 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1229(t1410, t605);
        let t7445 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1230(t1433, t71);
        let (t7457, t7458) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1231(t1874, t4028, t1458, t89);
    (t7412, t7415, t7416, t7423, t7426, t7428, t7431, t7432, t7435, t7445, t7457, t7458)
}
