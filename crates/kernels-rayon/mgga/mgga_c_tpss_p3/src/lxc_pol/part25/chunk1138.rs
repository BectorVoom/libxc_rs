//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1138/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1138(t15248: f64, t15251: f64, t15292: f64, t15294: f64, t15296: f64, t15299: f64, t15301: f64, t15304: f64, t15307: f64, t15309: f64, t15312: f64, t11938: f64, t12060: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64, t15334: f64, t15339: f64, t15342: f64) -> (f64, f64) {
    let t15669 = -0.69463333333333333333e-1_f64 * t15248 + 0.516475e0_f64 * t15251 + 0.3529725e1_f64 * t15292 + 0.6311625e0_f64 * t15294 + 0.23154444444444444445e-1_f64 * t15296 - 0.157790625e0_f64 * t15299 + 0.6311625e0_f64 * t15301 + 0.31558125e0_f64 * t15304 + 0.264729375e1_f64 * t15307 - 0.3529725e1_f64 * t15309 - 0.17648625e1_f64 * t15312;
    let t15690 = -0.20839e0_f64 * t15334 + 0.45908888888888888888e0_f64 * t11938 - t12060 - 0.34431666666666666667e0_f64 * t15283 + 0.46308888888888888889e-1_f64 * t15339 - 0.34731666666666666667e-1_f64 * t15342 - 0.68863333333333333334e0_f64 * t15268 - 0.20659e1_f64 * t15264 + 0.20659e1_f64 * t15277 + 0.309885e1_f64 * t15273 + 0.103295e1_f64 * t15288;
    (t15669, t15690)
}
