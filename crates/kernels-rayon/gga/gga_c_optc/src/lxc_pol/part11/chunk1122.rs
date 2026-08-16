//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1122/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1122(t172: f64, t4599: f64, t16572: f64, t1975: f64, t16606: f64, t740: f64, t16301: f64, t732: f64, t13053: f64, t1310: f64, t12997: f64, t13004: f64, t1320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48009 = t4599 * t172;
    let t48013 = t16572 * t1975;
    let t48017 = t16606 * t740;
    let t48024 = t732 * t16301;
    let t48028 = t1310 * t13053;
    let t48040 = t1310 * t12997;
    let t48045 = t13004 * t1320;
    (t48009, t48013, t48017, t48024, t48028, t48040, t48045)
}
