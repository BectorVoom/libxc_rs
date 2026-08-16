//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1113/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1113(t11938: f64, t11940: f64, t11941: f64, t11943: f64, t15239: f64, t15241: f64, t15243: f64, t15251: f64, t15259: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64, t9221: f64, t9243: f64) -> f64 {
    let t15291 = -t9243 + 4.0_f64 / 27.0_f64 * t9221 + 8.0_f64 / 27.0_f64 * t11938 + t11940 - t11941 - t11943 + 2.0_f64 / 27.0_f64 * t15239 + 10.0_f64 / 27.0_f64 * t15259 - 4.0_f64 / 3.0_f64 * t15264 - 4.0_f64 / 9.0_f64 * t15268 - 2.0_f64 / 9.0_f64 * t15241 + 2.0_f64 * t15273 + 4.0_f64 / 3.0_f64 * t15277 - t15243 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t15283 + 2.0_f64 / 3.0_f64 * t15288 + t15251 / 3.0_f64;
    t15291
}
