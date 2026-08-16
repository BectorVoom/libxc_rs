//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 741/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk741(t2920: f64, t8492: f64, t2925: f64, t1469: f64, t2891: f64, t1488: f64, t517: f64, t8356: f64, t3954: f64, t475: f64, t115: f64, t8379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8493 = t2920 * t8492;
    let t8494 = t8493 * t2925;
    let t8496 = t1469 * t2891;
    let t8498 = t1488 * t2891;
    let t8500 = t8356 * t517;
    let t8501 = t475 * t3954;
    let t8502 = t8500 * t8501;
    let t8504 = t8379 * t115;
    (t8493, t8494, t8496, t8498, t8502, t8504)
}
