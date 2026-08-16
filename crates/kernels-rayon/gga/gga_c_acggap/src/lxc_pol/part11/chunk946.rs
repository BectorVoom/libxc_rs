//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 946/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk946(t355: f64, t879: f64, t1095: f64, t7457: f64, t7458: f64, t2104: f64, t7780: f64, t2067: f64, t3073: f64, t1165: f64, t15407: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t31539 = t355 * t879;
    let t31542 = t7457 * t7458 * t1095 * t31539;
    let t31543 = 0.31448092289604152067e-3_f64 * t31542;
    let t31544 = t7780 * t2104;
    let t31562 = t3073 * t2067;
    let t31565 = t31562 * t1165 * t604 * t15407;
    (t31539, t31543, t31544, t31562, t31565)
}
