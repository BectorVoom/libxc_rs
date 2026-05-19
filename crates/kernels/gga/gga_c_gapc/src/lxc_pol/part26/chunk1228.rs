//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1228/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1228<F: Float>(t35112: F, t5218: F, t1044: F, t515: F, t169: F, t19: F, t3665: F, t116: F, t1882: F, t9092: F, t35169: F, t35173: F, t35177: F, t35182: F, t35184: F, t35186: F, t35188: F, t35190: F) -> (F, F) {
    let t35192 = t35112 * t5218;
    let t35194 = t515 * t1044;
    let t35197 = t169 * t35194 * t19 * t3665;
    let t35200 = t116 * t1882 * t9092;
    let t35202 = F::cast_from(0.59920486569434427612e-7_f64) * t35169 + F::cast_from(0.31675337336021900772e-5_f64) * t35173 - F::cast_from(0.54629306425871672463e-9_f64) * t35177 - F::cast_from(0.19263878310735033706e-7_f64) * t35182 - F::cast_from(0.34752370105806885418e-3_f64) * t35184 - F::cast_from(0.17376185052903442709e-3_f64) * t35186 + F::cast_from(0.17391272082782113042e-4_f64) * t35188 - F::cast_from(0.15716995342493974598e-7_f64) * t35190 - F::cast_from(0.33816362383187442026e-5_f64) * t35192 + F::cast_from(0.60724609375000000002e-3_f64) * t35197 - F::cast_from(0.71160893980376927166e-8_f64) * t35200;
    (t35194, t35202)
}
