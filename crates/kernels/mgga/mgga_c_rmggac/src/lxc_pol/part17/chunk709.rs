//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 709/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk709<F: Float>(t10093: F, t515: F, t7231: F, t3351: F, t2283: F, t8571: F, t551: F, t615: F) -> (F, F, F, F) {
    let t10094 = t515 * t10093;
    let t10095 = t7231 * t10094;
    let t10096 = t3351 * t10095;
    let t10097 = F::new(0.85129199786595678796e-5) * t10096;
    let t10098 = t8571 * t2283;
    let t10099 = F::new(0.85129199786595678796e-5) * t10098;
    let t10100 = t551 * t615;
    (t10095, t10097, t10099, t10100)
}
