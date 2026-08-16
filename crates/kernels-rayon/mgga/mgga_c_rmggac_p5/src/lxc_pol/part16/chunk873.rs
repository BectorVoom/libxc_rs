//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 873/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk873(t275: f64, t9596: f64, t1347: f64, t2475: f64, t41828: f64, t41882: f64, t41884: f64, t41922: f64, t41960: f64, t9658: f64, t41977: f64, t41979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43874 = 2.0_f64 * t275 * t9596;
    let t43877 = t1347 * t2475;
    let t43878 = 0.39726959900411316772e-4_f64 * t41828;
    let t43891 = 0.39726959900411316772e-4_f64 * t41882;
    let t43892 = 0.39726959900411316772e-4_f64 * t41884;
    let t43911 = 0.11918087970123395032e-3_f64 * t41922;
    let t43937 = 0.11918087970123395032e-3_f64 * t41960;
    let t43948 = 2.0_f64 * t275 * t9658;
    let t43956 = 0.3193131120497015617e0_f64 * t41977;
    let t43957 = 0.39726959900411316772e-4_f64 * t41979;
    (t43874, t43877, t43878, t43891, t43892, t43911, t43937, t43948, t43956, t43957)
}
