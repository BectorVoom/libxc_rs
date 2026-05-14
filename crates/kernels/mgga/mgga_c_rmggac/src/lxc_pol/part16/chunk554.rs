//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 554/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk554<F: Float>(t702: F, t934: F, t7303: F, t7307: F, t7318: F, t7339: F, t7342: F, t7383: F, t7391: F, t7395: F, t7402: F, t7415: F, t7430: F, t7438: F, t7582: F, t7594: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8048 = t934 * t702;
    let t8053 = 0.60975299583150056624e-3 * t7303;
    let t8054 = 0.60975299583150056624e-3 * t7307;
    let t8057 = 0.36366215538993788974e-1 * t7318;
    let t8069 = 0.60975299583150056624e-3 * t7339;
    let t8070 = 0.60975299583150056624e-3 * t7342;
    let t8081 = 0.15965655602485078085e0 * t7383;
    let t8083 = 0.86737941314158990616e-4 * t7391;
    let t8084 = 0.86737941314158990616e-4 * t7395;
    let t8086 = 0.39726959900411316772e-4 * t7402;
    let t8089 = 0.49658699875514145965e-4 * t7415;
    let t8092 = 0.39726959900411316772e-4 * t7430;
    let t8094 = 0.11918087970123395032e-3 * t7438;
    let t8125 = 0.29568125932752208315e-3 * t7582;
    let t8129 = 0.22223798384940648817e-1 * t7594;
    (t8048, t8053, t8054, t8057, t8069, t8070, t8081, t8083, t8084, t8086, t8089, t8092, t8094, t8125, t8129)
}
