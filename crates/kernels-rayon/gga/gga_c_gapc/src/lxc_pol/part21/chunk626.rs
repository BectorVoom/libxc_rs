//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 626/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk626(t1062: f64, t3729: f64, t3224: f64, t3643: f64, t2536: f64, t329: f64, t493: f64) -> (f64, f64, f64) {
    let t3730 = t1062 * t3729;
    let t3732 = t3643 * t3224;
    let t3734 = t493 * t329 * t2536;
    (t3730, t3732, t3734)
}
