//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk513;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk514;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk515;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk516;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk517;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta70(t1395: f64, t1401: f64, t577: f64, t671: f64, t582: f64, t586: f64, t589: f64, t593: f64, t596: f64, t600: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64, t65: f64, t43: f64, t46: f64, t48: f64, rho1: f64, sigma2: f64, t55: f64, t39: f64, t51: f64, t56: f64, t627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1404, t1406, t1408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk513(t1395, t1401, t577, t671, t582, t586, t589, t593, t596, t600, t4, t581);
        let t1409 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk514(t25, t28, t1408, zeta_threshold);
        let t1410 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk515(t1409, t31);
        let t1411 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk516(t1410, t65);
        let (t1414, t1417, t1419, t1420) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk517(t1409, t43, t46, t48, rho1, sigma2);
        let (t1423, t1426) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk518(t1409, t55, t1414, t1420, t39, t51, t56, t627);
    (t1404, t1406, t1408, t1409, t1410, t1411, t1414, t1417, t1419, t1420, t1423, t1426)
}
