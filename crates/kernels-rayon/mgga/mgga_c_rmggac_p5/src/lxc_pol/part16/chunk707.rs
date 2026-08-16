//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 707/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk707(t10267: f64, t903: f64, t2211: f64, t6522: f64, t739: f64, t9748: f64, t9752: f64, t9756: f64, t9763: f64, t9766: f64, t9770: f64, t9777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10268 = t903 * t10267;
    let t10269 = 0.35922725105591425692e0_f64 * t10268;
    let t10270 = t2211 * t6522;
    let t10271 = t739 * t10270;
    let t10272 = 0.23948483403727617128e0_f64 * t10271;
    let t10273 = 0.30487649791575028312e-3_f64 * t9748;
    let t10274 = 0.60975299583150056624e-3_f64 * t9752;
    let t10275 = 0.30487649791575028312e-3_f64 * t9756;
    let t10283 = 0.68186654135613354325e-2_f64 * t9763;
    let t10284 = 0.5987120850931904282e-1_f64 * t9766;
    let t10287 = 0.17961362552795712846e0_f64 * t9770;
    let t10288 = 0.85129199786595678799e-5_f64 * t9777;
    (t10269, t10270, t10272, t10273, t10274, t10275, t10283, t10284, t10287, t10288)
}
