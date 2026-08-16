//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1005/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1005(t50: f64, t1897: f64, t1900: f64, t1940: f64, t22034: f64, t22035: f64, t22041: f64, t22046: f64, t611: f64, t6547: f64, t6551: f64, t6554: f64, t22032: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t22050 = piecewise3(t51, 0.0_f64, -56.0_f64 / 81.0_f64 * t22034 * t22035 + 16.0_f64 / 9.0_f64 * t6547 * t1897 * t1900 - 2.0_f64 / 3.0_f64 * t1940 * t22041 - 8.0_f64 / 9.0_f64 * t6551 * t6554 + 2.0_f64 / 3.0_f64 * t611 * t22046);
    let t22052 = t22032 / 2.0_f64 + t22050 / 2.0_f64;
    t22052
}
