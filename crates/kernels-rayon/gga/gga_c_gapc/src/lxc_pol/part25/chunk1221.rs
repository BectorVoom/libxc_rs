//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1221/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1221(t11503: f64, t9041: f64, t11387: f64, t3060: f64, t3123: f64, t1423: f64, t3115: f64, t3116: f64, t11388: f64, t9050: f64, t34366: f64, t5727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34380 = t9041 * t11503;
    let t34382 = t3060 * t11387;
    let t34383 = t34382 * t3123;
    let t34386 = t3115 * t1423 * t3116;
    let t34388 = t11388 * t9050;
    let t34390 = t34366 * t5727;
    (t34380, t34382, t34383, t34386, t34388, t34390)
}
