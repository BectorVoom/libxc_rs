//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 366/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk366(t424: f64, t457: f64, t41: f64, t410: f64, t425: f64, t2: f64, t464: f64, t1381: f64, t1497: f64, t453: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1512 = t424 * t457;
    let t1513 = t41 * t1512;
    let t1515 = t410 * t425;
    let t1520 = t424 * t2;
    let t1521 = t1520 * t464;
    let t1524 = t1497 * t1381 * t453;
    let t1525 = t234 * t1524;
    let t1526 = 0.11696447245269292414e1_f64 * t1525;
    (t1512, t1513, t1515, t1520, t1521, t1524, t1526)
}
