//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1053/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1053(t47917: f64, t7717: f64, t3351: f64, t498: f64, t515: f64, t6522: f64, t7248: f64, t26287: f64, t46394: f64, t46385: f64, t30204: f64, t46388: f64) -> (f64, f64, f64, f64, f64) {
    let t47918 = t7717 * t47917;
    let t47923 = t3351 * t7248 * t515 * t6522 * t498;
    let t47931 = t26287 * t46394;
    let t47933 = t26287 * t46385;
    let t47935 = t30204 * t46388;
    (t47918, t47923, t47931, t47933, t47935)
}
