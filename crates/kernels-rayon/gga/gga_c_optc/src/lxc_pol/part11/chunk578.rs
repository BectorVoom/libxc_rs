//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 578/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk578(t1923: f64, t193: f64, t195: f64, t197: f64, t3573: f64, t4599: f64, t4752: f64, t750: f64, t201: f64, t5: f64, t743: f64) -> (f64, f64, f64) {
    let t4756 = -t1923 + 400.0_f64 / 27.0_f64 * t3573 - 25.0_f64 / 9.0_f64 * t193 * t195 * t4599 * t197 - 25.0_f64 / 9.0_f64 * t193 * t750 * t4752;
    let t4758 = t5 * t4756 * t201;
    let t4759 = t743 * t4758;
    (t4756, t4758, t4759)
}
