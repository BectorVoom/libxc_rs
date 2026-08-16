//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 680/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk680(t1328: f64, t2416: f64, t6320: f64, t1529: f64, t888: f64, t1217: f64, t885: f64, t1222: f64, t1210: f64, t78: f64, t119: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6321 = t2416 * t1328;
    let t6322 = t6320 * t6321;
    let t6325 = t1529 * t888;
    let t6328 = t1217 * t885;
    let t6334 = t1222 * t885;
    let t6336 = t78 * t1210;
    let t6338 = t481 * t6336 * t119;
    (t6321, t6322, t6325, t6328, t6334, t6338)
}
