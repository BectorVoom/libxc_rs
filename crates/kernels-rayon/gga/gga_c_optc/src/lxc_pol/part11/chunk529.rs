//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 529/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk529(t1036: f64, t1446: f64, t1085: f64, t1476: f64, t1483: f64, t3061: f64, t1587: f64, t2251: f64, t429: f64) -> (f64, f64, f64, f64) {
    let t4144 = t1446 * t1036;
    let t4182 = t1476 * t1085;
    let t4208 = t1483 * t3061;
    let t4215 = t2251 * t429 * t1587;
    (t4144, t4182, t4208, t4215)
}
