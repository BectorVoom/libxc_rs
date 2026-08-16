//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1151/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1151(t191: f64, t529: f64, t864: f64, t2437: f64, t2433: f64, t10127: f64, t10615: f64, t2364: f64, t23821: f64, t23823: f64, t23825: f64, t23946: f64, t23951: f64, t2544: f64, t2551: f64, t2722: f64, t3608: f64, t4038: f64, t4044: f64, t7180: f64, t7263: f64, t7301: f64, t7304: f64) -> f64 {
    let t23957 = t529 * t864 * t191;
    let t23958 = t23957 * t2437;
    let t23959 = t2433 * t23958;
    let t23963 = -t23821 + 8.0_f64 / 9.0_f64 * t23823 - 8.0_f64 * t4038 * t3608 * t10615 * t23825 + t7263 * t2544 + 128.0_f64 / 9.0_f64 * t2364 * t7301 - 64.0_f64 / 9.0_f64 * t7304 * t2551 - t23946 + 6.0_f64 * t4038 * t2722 * t4044 * t23825 + 8.0_f64 / 9.0_f64 * t4038 * t3608 * t4044 * t23951 - 400.0_f64 / 243.0_f64 * t23959 + 32.0_f64 / 3.0_f64 * t10127 * t7180;
    t23963
}
