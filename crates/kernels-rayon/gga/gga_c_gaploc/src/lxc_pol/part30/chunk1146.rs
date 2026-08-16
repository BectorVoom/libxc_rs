//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1146/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1146(t20954: f64, t3196: f64, t1407: f64, t9445: f64, t1328: f64, t20550: f64, t6914: f64, t9438: f64, t1429: f64, t549: f64, t9572: f64, t1323: f64, t7033: f64, t9439: f64) -> (f64, f64, f64, f64, f64) {
    let t30901 = t20954 * t3196;
    let t30902 = 0.38342925953920749676e0_f64 * t30901;
    let t30903 = t1407 * t9445;
    let t30907 = t6914 * t9438 * t20550 * t1328;
    let t30920 = 0.11916829983950142223e0_f64 * t1429 * t549 * t9572;
    let t30923 = t7033 * t9438 * t9439 * t1323;
    (t30902, t30903, t30907, t30920, t30923)
}
