//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 700/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk700(t10093: f64, t515: f64, t7231: f64, t3351: f64, t2283: f64, t8571: f64, t551: f64, t615: f64) -> (f64, f64, f64, f64) {
    let t10094 = t515 * t10093;
    let t10095 = t7231 * t10094;
    let t10096 = t3351 * t10095;
    let t10098 = t8571 * t2283;
    let t10100 = t551 * t615;
    (t10095, t10096, t10098, t10100)
}
