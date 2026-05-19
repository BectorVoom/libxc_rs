//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 709/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk709<F: Float>(t8467: F, t8470: F, t8477: F, t8484: F, t8488: F, t8492: F, t8500: F, t8534: F, t8538: F, t8657: F, t9408: F, t8692: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10265 = F::cast_from(0.1440846329149835838e-2_f64) * t8467;
    let t10266 = F::cast_from(0.20496175532535769482e-3_f64) * t8470;
    let t10276 = F::cast_from(0.60975299583150056624e-3_f64) * t8477;
    let t10277 = F::cast_from(0.86737941314158990616e-4_f64) * t8484;
    let t10278 = F::cast_from(0.60975299583150056624e-3_f64) * t8488;
    let t10279 = F::cast_from(0.86737941314158990616e-4_f64) * t8492;
    let t10280 = F::cast_from(0.39726959900411316772e-4_f64) * t8500;
    let t10285 = F::cast_from(0.36366215538993788974e-1_f64) * t8534;
    let t10286 = F::cast_from(0.10909864661698136692e0_f64) * t8538;
    let t10331 = F::cast_from(0.36366215538993788974e-1_f64) * t8657;
    let t10356 = F::new(0.4726e1) * t9408;
    let t10357 = F::cast_from(0.39726959900411316772e-4_f64) * t8692;
    (t10265, t10266, t10276, t10277, t10278, t10279, t10280, t10285, t10286, t10331, t10356, t10357)
}
