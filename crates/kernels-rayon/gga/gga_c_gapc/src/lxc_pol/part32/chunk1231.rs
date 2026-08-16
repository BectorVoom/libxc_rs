//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1231/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1231(t35112: f64, t5218: f64, t1044: f64, t515: f64, t169: f64, t19: f64, t3665: f64, t116: f64, t1882: f64, t9092: f64, t35169: f64, t35173: f64, t35177: f64, t35182: f64, t35184: f64, t35186: f64, t35188: f64, t35190: f64) -> (f64, f64) {
    let t35192 = t35112 * t5218;
    let t35194 = t515 * t1044;
    let t35197 = t169 * t35194 * t19 * t3665;
    let t35200 = t116 * t1882 * t9092;
    let t35202 = 0.59920486569434427612e-7_f64 * t35169 + 0.31675337336021900772e-5_f64 * t35173 - 0.54629306425871672463e-9_f64 * t35177 - 0.19263878310735033706e-7_f64 * t35182 - 0.34752370105806885418e-3_f64 * t35184 - 0.17376185052903442709e-3_f64 * t35186 + 0.17391272082782113042e-4_f64 * t35188 - 0.15716995342493974598e-7_f64 * t35190 - 0.33816362383187442026e-5_f64 * t35192 + 0.60724609375000000002e-3_f64 * t35197 - 0.71160893980376927166e-8_f64 * t35200;
    (t35194, t35202)
}
