//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 514/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk514(t106: f64, t2330: f64, t2333: f64, t97: f64, t1421: f64, t1424: f64, t1459: f64, t1463: f64, t1470: f64, t1480: f64, t1488: f64, t1511: f64, t1514: f64, t1516: f64, t1519: f64, t1522: f64, t1526: f64, t1529: f64, t1533: f64, t2328: f64) -> f64 {
    let t2335 = t97 * t106 * t2330 * t2333;
    let t2336 = -t1421 + t1424 - t1511 - t1519 + t1459 - t1526 - t1514 + t1516 + t1470 - t1480 - t1488 + 2.0_f64 * t2328 - t1529 + t2335 + t1463 + t1522 - t1533;
    t2336
}
