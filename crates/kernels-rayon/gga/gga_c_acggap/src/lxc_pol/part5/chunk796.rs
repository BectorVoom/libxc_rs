//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 796/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk796(t1750: f64, t3379: f64, t1165: f64, t1748: f64, t4289: f64, t1298: f64, t157: f64) -> (f64, f64, f64) {
    let t6252 = t3379 * t1750;
    let t6255 = t1165 * t4289 * t1748;
    let t6258 = t157 * t1298;
    (t6252, t6255, t6258)
}
