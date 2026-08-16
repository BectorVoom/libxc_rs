//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1214/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1214(t50: f64, t13020: f64, t16241: f64, t1940: f64, t22034: f64, t3339: f64, t4573: f64, t55917: f64, t55922: f64, t55927: f64, t611: f64, t55916: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t55931 = piecewise3(t51, 0.0_f64, -56.0_f64 / 81.0_f64 * t22034 * t55917 + 16.0_f64 / 9.0_f64 * t13020 * t4573 - 2.0_f64 / 3.0_f64 * t1940 * t55922 - 8.0_f64 / 9.0_f64 * t3339 * t16241 + 2.0_f64 / 3.0_f64 * t611 * t55927);
    let t55933 = t55916 / 2.0_f64 + t55931 / 2.0_f64;
    t55933
}
