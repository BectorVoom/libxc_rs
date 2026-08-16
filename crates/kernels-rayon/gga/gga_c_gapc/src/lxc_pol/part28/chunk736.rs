//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 736/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk736(t3949: f64, t436: f64, t8459: f64, t1476: f64, t2945: f64, t126: f64, t505: f64, t568: f64, t120: f64, t152: f64, t493: f64, t5918: f64) -> (f64, f64, f64, f64, f64) {
    let t8460 = t436 * t3949;
    let t8461 = t8459 * t8460;
    let t8463 = t1476 * t2945;
    let t8465 = t126 * t505;
    let t8466 = t8465 * t568;
    let t8467 = t120 * t8466;
    let t8469 = t493 * t152;
    let t8470 = t8469 * t5918;
    (t8461, t8463, t8465, t8467, t8470)
}
