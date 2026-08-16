//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 848/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk848(t6337: f64, t1395: f64, t1805: f64, t5572: f64, t1378: f64, t226: f64, t5577: f64, t1708: f64, t228: f64, t1396: f64, t1707: f64, t1809: f64, t253: f64, t5571: f64, t5834: f64, t6135: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6338 = param_beta * t6337;
    let t6342 = t1805 * t1395;
    let t6343 = t5572 * t6342;
    let t6348 = t5577 * t1805 * t1378 * t226;
    let t6351 = t1708 * t228 * t6337;
    let t6353 = -t1396 * t5834 - t1707 * t6351 - t1809 * t6135 + t253 * t6338 + 2.0_f64 * t5571 * t6343 + t5571 * t6348;
    (t6338, t6342, t6343, t6348, t6351, t6353)
}
