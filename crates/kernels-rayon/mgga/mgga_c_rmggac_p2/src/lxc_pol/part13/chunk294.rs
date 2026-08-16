//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 294/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk294(t1494: f64, t209: f64, t469: f64, t6: f64, t1193: f64, t1466: f64, t476: f64, t605: f64, t221: f64, t589: f64, t1228: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1497 = t469 * t6 * t1494 * t209;
    let t1500 = t1193 * t1466;
    let t1501 = t605 * t476;
    let t1502 = t1501 * t209;
    let t1503 = t221 * t1502;
    let t1508 = t589 * t476;
    let t1509 = t1508 * t209;
    let t1510 = t221 * t1509;
    let t1513 = t1228 * t612;
    (t1497, t1500, t1501, t1502, t1503, t1508, t1510, t1513)
}
