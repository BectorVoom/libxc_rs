//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 218/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk218(t43: f64, t50: f64, t601: f64, t603: f64, t103: f64, t172: f64, t47: f64, t549: f64, t52: f64, t553: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t605 = 0.58482233974552040708e0_f64 * t601 * t603;
    let t606 = t103 * t172;
    let t607 = 1.0_f64 / t47;
    let t610 = piecewise3(t44, 0.0_f64, 2.0_f64 / 3.0_f64 * t607 * t549);
    let t611 = 1.0_f64 / t52;
    let t614 = piecewise3(t51, 0.0_f64, 2.0_f64 / 3.0_f64 * t611 * t553);
    let t616 = t610 / 2.0_f64 + t614 / 2.0_f64;
    (t605, t606, t607, t611, t616)
}
