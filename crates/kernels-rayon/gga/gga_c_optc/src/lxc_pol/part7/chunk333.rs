//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 333/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk333(t398: f64, t393: f64, t1023: f64, t1049: f64, t1030: f64, t1041: f64, t1046: f64, t1053: f64) -> (f64, f64, f64, f64) {
    let t1065 = t398 * t398;
    let t1066 = 1.0_f64 / t1065;
    let t1067 = t393 * t1066;
    let t1069 = 0.516475e0_f64 * t1023;
    let t1072 = 0.104195e0_f64 * t1049;
    let t1074 = 0.3529725e1_f64 * t1041 - t1069 - 0.516475e0_f64 * t1030 + 0.6311625e0_f64 * t1046 - t1072 - 0.104195e0_f64 * t1053;
    (t1065, t1066, t1067, t1074)
}
