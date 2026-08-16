//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1163/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1163(t3708: f64, t8863: f64, t3714: f64, t11450: f64, t11451: f64, t21115: f64, t11512: f64, t1736: f64, t1743: f64, t1749: f64, t1: f64, t26662: f64) -> (f64, f64, f64, f64, f64) {
    let t34409 = t8863 * t3708;
    let t34410 = t34409 * t3714;
    let t34413 = t11450 * t11451 * t21115;
    let t34417 = t1743 * t11512 * t1736 * t1749;
    let t34419 = t26662 * t1;
    (t34409, t34410, t34413, t34417, t34419)
}
