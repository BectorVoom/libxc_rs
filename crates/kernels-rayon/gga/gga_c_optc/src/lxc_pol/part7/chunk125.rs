//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 125/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk125(t277: f64, t124: f64, t234: f64, t269: f64, t271: f64) -> (f64, f64) {
    let t293 = 1.0_f64 / t277;
    let t297 = f64::exp(-0.12897460341341234505e3_f64 * (-t234 + t269 + t271) * t293 * t124);
    (t293, t297)
}
