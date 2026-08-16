//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 396/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk396(t465: f64, t519: f64, t106: f64, t518: f64, t101: f64, t525: f64) -> (f64, f64, f64, f64, f64) {
    let t1564 = t465 * t519;
    let t1568 = t518 * t106;
    let t1569 = 1.0_f64 / t1568;
    let t1570 = t101 * t1569;
    let t1571 = t525 * t525;
    (t1564, t1568, t1569, t1570, t1571)
}
