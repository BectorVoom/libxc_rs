//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1161/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1161(t1206: f64, t3200: f64, t338: f64, t14185: f64, t3212: f64, t9283: f64, t4227: f64, t938: f64, t2409: f64, t3067: f64, t1161: f64, t353: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15018 = t338 * t3200 * t1206;
    let t15021 = t14185 * t3212;
    let t15022 = t9283 * t15021;
    let t15025 = t4227 * t938;
    let t15027 = t2409 * t3067 * t15025;
    let t15034 = t14185 * t1161;
    let t15035 = t353 * t15034;
    (t15018, t15021, t15022, t15025, t15027, t15034, t15035)
}
