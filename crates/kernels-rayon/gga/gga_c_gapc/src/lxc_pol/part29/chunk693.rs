//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 693/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk693(t286: f64, t818: f64, t442: f64, t7592: f64, t2394: f64, t825: f64, t2723: f64, t918: f64, t1018: f64, t2520: f64, t1044: f64, t125: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7593 = t818 * t286;
    let t7595 = t7592 * t7593 * t442;
    let t7624 = t2394 * t825;
    let t7626 = t918 * t2723;
    let t7675 = t2520 * t1018;
    let t7676 = t1044 * t125;
    (t7593, t7595, t7624, t7626, t7675, t7676)
}
