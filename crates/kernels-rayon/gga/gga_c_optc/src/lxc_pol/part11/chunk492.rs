//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 492/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk492(t115: f64, t2770: f64, t426: f64, t3209: f64, t1724: f64, t1168: f64, t442: f64, t1120: f64) -> (f64, f64, f64, f64) {
    let t3211 = t426 * t2770 * t115;
    let t3212 = t3209 * t3211;
    let t3217 = t1724 * t3211;
    let t3233 = t1168 * t442;
    let t3234 = t3233 * t1120;
    (t3212, t3217, t3233, t3234)
}
