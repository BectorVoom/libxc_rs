//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1030/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1030(t1864: f64, t236: f64, t498: f64, t7231: f64, t8517: f64, t321: f64, t3352: f64, t1971: f64, t333: f64, t511: f64, t352: f64, t515: f64) -> (f64, f64, f64, f64) {
    let t46933 = t8517 * t7231 * t236 * t1864 * t498;
    let t46938 = t8517 * t3352 * t236 * t1864 * t321;
    let t46943 = t8517 * t1971 * t511 * t1864 * t333;
    let t46948 = t8517 * t1971 * t515 * t1864 * t352;
    (t46933, t46938, t46943, t46948)
}
