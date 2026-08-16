//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1407/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1407(t28030: f64, t1220: f64, t176: f64, t26175: f64, t26179: f64, t26282: f64, t26337: f64, t275: f64, t277: f64, t28002: f64, t28010: f64, t28017: f64, t28020: f64, t28023: f64, t28026: f64, t28028: f64, t3274: f64, t3284: f64, t498: f64, t8431: f64, t8436: f64, t914: f64, t95: f64, sigma2: f64) -> f64 {
    let t28031 = 1.0_f64 / t28030;
    let t28039 = t176 * t28002 * t275 * sigma2 * t498 / 2.0_f64 + 56.0_f64 / 27.0_f64 * t3274 * t8431 + 140.0_f64 / 81.0_f64 * t1220 * t914 * t28010 * t26337 + 2.0_f64 / 3.0_f64 * t3274 * t8436 - 4.0_f64 / 9.0_f64 * t28017 + 20.0_f64 / 81.0_f64 * t28020 - 2.0_f64 / 9.0_f64 * t28023 - 8.0_f64 / 27.0_f64 * t28026 - t26175 - t26179 - 0.15506928860942058298e-1_f64 * t95 * t277 * t28028 * t28031 + 8.0_f64 * t1220 * t914 * t3284 * t26282;
    t28039
}
