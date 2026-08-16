//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 475/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk475(t2619: f64, t612: f64, t891: f64, t918: f64, t617: f64, t1695: f64, t933: f64, t327: f64, t442: f64, t6: f64, t786: f64, t1087: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2620 = t2619 * t612;
    let t2621 = t918 * t891;
    let t2622 = t617 * t2621;
    let t2625 = t933 * t1695;
    let t2626 = t442 * t327;
    let t2627 = t786 * t6;
    let t2628 = t1087 * t2627;
    (t2620, t2621, t2622, t2625, t2626, t2627, t2628)
}
