//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 341/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk341(t6: f64, t644: f64, t101: f64, t517: f64, t423: f64, t462: f64, t472: f64, t513: f64, t1468: f64, t465: f64, t1427: f64, t433: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1482 = t6 * t644;
    let t1483 = t1482 * t101;
    let t1484 = t1483 * t517;
    let t1487 = t462 * t423;
    let t1488 = t1487 * t472;
    let t1491 = t513 * t423;
    let t1492 = t1491 * t517;
    let t1495 = t1468 * t465;
    let t1498 = t1427 * t433;
    (t1482, t1484, t1487, t1488, t1492, t1495, t1498)
}
