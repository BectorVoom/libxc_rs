//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk493;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk494;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk495;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk496;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta72(t1409: f64, t31: f64, t65: f64, t43: f64, t46: f64, t48: f64, rho1: f64, sigma2: f64, t55: f64, t39: f64, t51: f64, t56: f64, t627: f64, t33: f64, t634: f64, t638: f64, t72: f64, t66: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1410 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk493(t1409, t31);
        let (t1411, t1414, t1419) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk494(t1410, t65, t1409, t43, t46, t48, rho1);
        let t1420 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk495(t1419, sigma2);
        let (t1426, t1427, t1430, t1431, t1433) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk496(t1409, t55, t1414, t1420, t39, t51, t56, t627, t33, t634, t638);
        let (t1434, t1437) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk497(t1433, t72, t1411, t1427, t66, t80);
    (t1410, t1411, t1414, t1419, t1420, t1426, t1427, t1430, t1431, t1433, t1434, t1437)
}
