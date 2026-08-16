//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 128/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk128(t167: f64, t180: f64, t249: f64, t361: f64, t380: f64, t396: f64, t403: f64, t411: f64, t418: f64, t5: f64) -> f64 {
    let t421 = 0.53237641966666666666e-3_f64 * t5 * t249 * t167 + 1.0_f64 * t396 * t403 - t361 - t380 + 0.18311447306006545054e-3_f64 * t5 * t249 * t180 + 0.5848223622634646207e0_f64 * t411 * t418;
    t421
}
