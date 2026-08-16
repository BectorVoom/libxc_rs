//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 379/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk379(t1376: f64, t86: f64, t2: f64, t424: f64, t464: f64, t1381: f64, t1497: f64, t453: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1518 = t1376 * t86;
    let t1519 = 0.19751673498613801407e-1_f64 * t1518;
    let t1520 = t424 * t2;
    let t1521 = t1520 * t464;
    let t1522 = 0.36622894612013090108e-3_f64 * t1521;
    let t1524 = t1497 * t1381 * t453;
    let t1525 = t234 * t1524;
    let t1526 = 0.11696447245269292414e1_f64 * t1525;
    (t1519, t1520, t1521, t1522, t1524, t1526)
}
