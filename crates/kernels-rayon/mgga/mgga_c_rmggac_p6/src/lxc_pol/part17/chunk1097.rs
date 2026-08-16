//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1097/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1097(t26287: f64, t46394: f64, t46385: f64, t30204: f64, t46388: f64, t1502: f64, t16503: f64, t16504: f64, t552: f64, t10078: f64, t34761: f64, t34962: f64, t8420: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47931 = t26287 * t46394;
    let t47933 = t26287 * t46385;
    let t47935 = t30204 * t46388;
    let t47946 = t16503 * t16504 * t552 * t1502;
    let t47948 = t34761 * t10078;
    let t47952 = t16503 * t34962 * t552 * t8420;
    (t47931, t47933, t47935, t47946, t47948, t47952)
}
