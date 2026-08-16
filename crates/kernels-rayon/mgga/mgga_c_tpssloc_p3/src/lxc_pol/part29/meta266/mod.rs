//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1252;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1253;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1254;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta266(t577: f64, t671: f64, t7014: f64, t7017: f64, t7019: f64, t7415: f64, t7423: f64, t33: f64, t3953: f64, t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1433: f64, t71: f64, t1874: f64, t4028: f64, t1458: f64, t89: f64, t1774: f64, t1873: f64, t109: f64, t652: f64, t1453: f64, t6530: f64, t6529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7426, t7428, t7431, t7432) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1252(t577, t671, t7014, t7017, t7019, t7415, t7423, t33, t3953, t1437, t79, t72);
        let (t7435, t7445, t7457, t7458) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1253(t1410, t605, t1433, t71, t1874, t4028, t1458, t89);
        let (t7460, t7461) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1254(t1874, t7458, t1774, t1873);
        let (t7463, t7467) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1255(t109, t652, t7461, t1453, t6530, t6529);
    (t7426, t7428, t7431, t7432, t7435, t7445, t7457, t7458, t7460, t7461, t7463, t7467)
}
