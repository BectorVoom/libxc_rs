//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 551/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk551(t1179: f64, t4380: f64, t1502: f64, t2367: f64, t1162: f64, t1168: f64, t1515: f64, t871: f64) -> (f64, f64, f64, f64) {
    let t4486 = t1179 * t4380;
    let t4488 = t2367 * t1502;
    let t4489 = t1162 * t4488;
    let t4492 = t1168 * t1515 * t871;
    (t4486, t4488, t4489, t4492)
}
