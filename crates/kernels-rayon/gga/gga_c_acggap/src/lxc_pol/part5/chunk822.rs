//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 822/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk822(t6421: f64, t6441: f64, t6453: f64, t6574: f64, t105: f64, t469: f64, t96: f64, t1298: f64, t1670: f64, t694: f64, t1717: f64, t814: f64) -> (f64, f64, f64, f64) {
    let t6576 = t6421 + t6441 + t6453 + t6574;
    let t6579 = t96 * t105 * t6576 * t469;
    let t6581 = t694 * t1670 * t1298;
    let t6583 = t1717 * t814;
    (t6576, t6579, t6581, t6583)
}
