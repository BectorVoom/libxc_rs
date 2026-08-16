//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 338/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk338(t1410: f64, t65: f64, t1409: f64, t43: f64, t46: f64, t48: f64, t55: f64, t39: f64, t51: f64, t56: f64, t627: f64, t33: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t1411 = t1410 * t65;
    let t1414 = t43 * t1409;
    let t1417 = t46 * rho1;
    let t1419 = 1.0_f64 / t48 / t1417;
    let t1420 = sigma2 * t1419;
    let t1423 = t55 * t1409;
    let t1426 = 5.0_f64 / 6.0_f64 * t39 * t1414 - 8.0_f64 / 3.0_f64 * t1420 * t56 - 5.0_f64 / 6.0_f64 * t51 * t1423 + t627;
    let t1427 = t33 * t1426;
    (t1411, t1414, t1420, t1426, t1427)
}
