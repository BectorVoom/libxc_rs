//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 173/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk173(t116: f64, t430: f64, t124: f64, t293: f64, t391: f64, t419: f64, t421: f64) -> (f64, f64) {
    let t431 = t116 * t430;
    let t438 = f64::exp(-0.12897460341341234505e3_f64 * (-t391 + t419 + t421) * t293 * t124);
    (t431, t438)
}
