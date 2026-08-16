//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1077/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1077(t188: f64, t4758: f64, t6680: f64, t13053: f64, t732: f64, t13113: f64, t1983: f64, t4743: f64, t1320: f64, t9534: f64, t12997: f64, t2229: f64, t4744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37228 = t188 * t6680 * t4758;
    let t37258 = t732 * t13053;
    let t37294 = t13113 * t1983;
    let t37325 = t188 * t6680 * t4743;
    let t37328 = t9534 * t1320;
    let t37341 = t732 * t12997;
    let t37417 = t2229 * t4744;
    (t37228, t37258, t37294, t37325, t37328, t37341, t37417)
}
