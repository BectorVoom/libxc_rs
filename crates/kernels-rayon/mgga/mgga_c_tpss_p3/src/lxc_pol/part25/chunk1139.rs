//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1139/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1139(t11850: f64, t11875: f64, t11932: f64, t12024: f64, t12035: f64, t12046: f64, t15237: f64, t15239: f64, t15241: f64, t15243: f64, t15245: f64, t15259: f64, t15321: f64, t15324: f64, t15327: f64, t15330: f64, t15669: f64, t15690: f64, t9192: f64, t9221: f64, t9429: f64, t9438: f64) -> f64 {
    let t15692 = -t9429 + 0.11577222222222222222e0_f64 * t9192 - t12024 + 0.23154444444444444445e0_f64 * t11850 - t9438 + 0.104195e0_f64 * t15237 + 0.11477222222222222222e0_f64 * t15239 - 0.34431666666666666667e0_f64 * t15241 - 0.17215833333333333333e0_f64 * t15243 - 0.13892666666666666667e0_f64 * t15245 + t15669 + t12035 - 0.68863333333333333332e0_f64 * t11875 - t12046 + 0.22954444444444444444e0_f64 * t9221 + 0.4630888888888888889e-1_f64 * t11932 + 0.57386111111111111112e0_f64 * t15259 + 0.41678e0_f64 * t15321 + 0.62517e0_f64 * t15324 + 0.20839e0_f64 * t15327 - 0.69463333333333333334e-1_f64 * t15330 + t15690;
    t15692
}
