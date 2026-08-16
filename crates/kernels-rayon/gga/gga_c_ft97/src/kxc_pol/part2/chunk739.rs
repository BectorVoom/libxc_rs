//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 739/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk739(t18: f64, t432: f64, t1903: f64, t1902: f64, t492: f64, t1910: f64, t1909: f64, t363: f64, t3187: f64, t1882: f64, t3277: f64, t3273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11594 = t18 * t432;
    let t11595 = t1903 * t11594;
    let t11596 = t1902 * t11595;
    let t11599 = t18 * t492;
    let t11600 = t1910 * t11599;
    let t11601 = t1909 * t11600;
    let t11604 = t18 * t363;
    let t11605 = t3187 * t11604;
    let t11606 = t1909 * t11605;
    let t11610 = 2.0_f64 / 27.0_f64 * t1882 * t3277;
    let t11612 = 2.0_f64 / 9.0_f64 * t1882 * t3273;
    (t11596, t11601, t11604, t11606, t11610, t11612)
}
