//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 582/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk582(t702: f64, t934: f64, t7303: f64, t7307: f64, t7318: f64, t7339: f64, t7342: f64, t7383: f64, t7391: f64, t7395: f64, t7402: f64, t7415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8048 = t934 * t702;
    let t8053 = 0.60975299583150056624e-3_f64 * t7303;
    let t8054 = 0.60975299583150056624e-3_f64 * t7307;
    let t8057 = 0.36366215538993788974e-1_f64 * t7318;
    let t8069 = 0.60975299583150056624e-3_f64 * t7339;
    let t8070 = 0.60975299583150056624e-3_f64 * t7342;
    let t8081 = 0.15965655602485078085e0_f64 * t7383;
    let t8083 = 0.86737941314158990616e-4_f64 * t7391;
    let t8084 = 0.86737941314158990616e-4_f64 * t7395;
    let t8086 = 0.39726959900411316772e-4_f64 * t7402;
    let t8089 = 0.49658699875514145965e-4_f64 * t7415;
    (t8048, t8053, t8054, t8057, t8069, t8070, t8081, t8083, t8084, t8086, t8089)
}
