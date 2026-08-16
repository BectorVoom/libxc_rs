//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 325/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk325(t1409: f64, t55: f64, t1414: f64, t1420: f64, t39: f64, t51: f64, t56: f64, t627: f64, t33: f64, t634: f64, t638: f64) -> (f64, f64, f64, f64) {
    let t1423 = t55 * t1409;
    let t1426 = 5.0_f64 / 6.0_f64 * t39 * t1414 - 8.0_f64 / 3.0_f64 * t1420 * t56 - 5.0_f64 / 6.0_f64 * t51 * t1423 + t627;
    let t1427 = t33 * t1426;
    let t1430 = t634 * t1409;
    let t1431 = t638 * t1409;
    let t1433 = -4.0_f64 / 3.0_f64 * t1430 + 4.0_f64 / 3.0_f64 * t1431;
    (t1423, t1426, t1427, t1433)
}
