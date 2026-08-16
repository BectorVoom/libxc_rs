//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 383/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk383(t501: f64, t546: f64, t496: f64, t513: f64, t465: f64, t519: f64, t106: f64, t518: f64, t101: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1555 = 8.0_f64 * t501 * t546;
    let t1556 = t496 * t513;
    let t1559 = 8.0_f64 * t496 * t546;
    let t1564 = t465 * t519;
    let t1568 = t518 * t106;
    let t1569 = 1.0_f64 / t1568;
    let t1570 = t101 * t1569;
    let t1571 = t525 * t525;
    (t1555, t1556, t1559, t1564, t1568, t1569, t1570, t1571)
}
