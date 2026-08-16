//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 867/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk867(t39977: f64, t39997: f64, t40045: f64, t40062: f64, t40075: f64, t40084: f64, t40086: f64, t40088: f64, t40121: f64, t40259: f64, t9343: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43207 = 0.39726959900411316772e-4_f64 * t39977;
    let t43211 = 0.3193131120497015617e0_f64 * t39997;
    let t43241 = 0.11918087970123395032e-3_f64 * t40045;
    let t43267 = 0.39726959900411316772e-4_f64 * t40062;
    let t43270 = 0.49658699875514145965e-4_f64 * t40075;
    let t43272 = 0.3842256877732895568e-2_f64 * t40084;
    let t43273 = 0.3842256877732895568e-2_f64 * t40086;
    let t43274 = 0.3842256877732895568e-2_f64 * t40088;
    let t43288 = 0.11918087970123395032e-3_f64 * t40121;
    let t43338 = 0.36366215538993788974e-1_f64 * t40259;
    let t43366 = 0.4726e1_f64 * t942 * t9343;
    (t43207, t43211, t43241, t43267, t43270, t43272, t43273, t43274, t43288, t43338, t43366)
}
