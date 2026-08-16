//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1290/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1290(t10298: f64, t4342: f64, t7324: f64, t9034: f64, t6571: f64, t8045: f64, t10301: f64, t4349: f64, t605: f64, t10802: f64, t14537: f64, t1383: f64, t17293: f64, t3366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34008 = 4.0_f64 * t4342 * t10298;
    let t34010 = 2.0_f64 * t7324 * t9034;
    let t34012 = 2.0_f64 * t8045 * t6571;
    let t34018 = 12.0_f64 * t4349 * t10301 * t605;
    let t34020 = 12.0_f64 * t14537 * t10802;
    let t34023 = 24.0_f64 * t17293 * t3366 * t1383;
    (t34008, t34010, t34012, t34018, t34020, t34023)
}
