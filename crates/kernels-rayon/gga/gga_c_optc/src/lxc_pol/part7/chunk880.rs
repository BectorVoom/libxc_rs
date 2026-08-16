//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 880/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk880(t1122: f64, t8446: f64, t3120: f64, t3116: f64, t3117: f64, t3126: f64, t2860: f64, t3119: f64, t3118: f64, t22: f64, t3145: f64, t2850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8447 = t8446 * t1122;
    let t8448 = t8447 * t3120;
    let t8449 = t3116 * t8448;
    let t8451 = t3117 * t3126;
    let t8452 = t8451 * t3120;
    let t8455 = t3119 * t2860;
    let t8456 = t3118 * t8455;
    let t8459 = t22 * t3145;
    let t8460 = t8459 * t1122;
    let t8461 = t3119 * t2850;
    (t8447, t8449, t8451, t8452, t8455, t8456, t8459, t8460, t8461)
}
