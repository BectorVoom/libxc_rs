//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk489;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk490;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk491;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk492;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk493;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta73(t1410: f64, t65: f64, t1409: f64, t43: f64, t46: f64, t48: f64, rho1: f64, sigma2: f64, t55: f64, t39: f64, t51: f64, t56: f64, t627: f64, t33: f64, t634: f64, t638: f64, t72: f64, t66: f64, t80: f64, t5: f64, t1406: f64, t605: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1411, t1414, t1417, t1419, t1420) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk489(t1410, t65, t1409, t43, t46, t48, rho1, sigma2);
        let (t1423, t1426) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk490(t1409, t55, t1414, t1420, t39, t51, t56, t627);
        let (t1427, t1433) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk491(t1426, t33, t1409, t634, t638);
        let t1434 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk492(t1433, t72);
        let t1437 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk493(t1411, t1427, t1434, t66, t80);
        let t1441 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk494(t5, t1406, t1437, t605, t86);
    (t1411, t1414, t1417, t1419, t1420, t1423, t1426, t1427, t1433, t1434, t1437, t1441)
}
