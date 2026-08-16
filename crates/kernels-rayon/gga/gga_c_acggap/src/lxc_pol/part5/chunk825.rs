//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 825/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk825(t1679: f64, t467: f64, t6614: f64, t1713: f64, t192: f64, t301: f64, t96: f64, t695: f64, t1674: f64, t1662: f64, t1680: f64, t130: f64, t595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6616 = t1679 * t6614 * t467;
    let t6619 = t96 * t301 * t192 * t1713;
    let t6621 = t695 * t1713;
    let t6622 = t1674 * t6621;
    let t6625 = t1679 * t1680 * t1662;
    let t7309 = t130 * t595;
    (t6616, t6619, t6621, t6622, t6625, t7309)
}
