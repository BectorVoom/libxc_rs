//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 670/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk670(t1859: f64, t1867: f64, t586: f64, t1757: f64, t1784: f64, t535: f64, t31: f64, t3648: f64, t4: f64, t14: f64, t2: f64, t25: f64) -> (f64, f64, f64, f64, f64) {
    let t6347 = t1859 * t1867;
    let t6348 = t6347 * t586;
    let t6356 = 6.0_f64 * t1757 * t535 * t1784;
    let t6359 = 0.34451131037037037036e-2_f64 * t4 * t3648 * t31;
    let t6363 = 1.0_f64 / t14 / t25 * t2 / 4.0_f64;
    (t6347, t6348, t6356, t6359, t6363)
}
