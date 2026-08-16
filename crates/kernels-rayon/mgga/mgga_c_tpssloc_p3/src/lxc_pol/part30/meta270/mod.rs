//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta270 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1222;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1223;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1224;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1225;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1226;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1227;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1228;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta270(t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1409: f64, t6500: f64, t6503: f64, t67: f64, t1864: f64, t1433: f64, t71: f64, t1863: f64, t5: f64, t1860: f64, t1865: f64, t6490: f64, t7428: f64, t112: f64, t1874: f64, t4028: f64, t1458: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7431, t7432) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1222(t1437, t79, t72);
        let t7435 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1223(t1410, t605);
        let (t7440, t7441) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1224(t1409, t6500, t6503, t67);
        let t7442 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1225(t1864, t7441);
        let t7445 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1226(t1433, t71);
        let t7446 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1227(t1863, t7445);
        let (t7450, t7451) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1228(t5, t1860, t1865, t6490, t7428, t7432, t7435, t7442, t7446, t112);
        let (t7457, t7458) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1229(t1874, t4028, t1458, t89);
    (t7431, t7432, t7435, t7440, t7441, t7442, t7445, t7446, t7450, t7451, t7457, t7458)
}
