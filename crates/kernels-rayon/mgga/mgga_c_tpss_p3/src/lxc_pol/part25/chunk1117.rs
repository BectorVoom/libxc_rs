//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1117/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1117(t11850: f64, t11875: f64, t11932: f64, t12093: f64, t12104: f64, t12115: f64, t15237: f64, t15239: f64, t15241: f64, t15243: f64, t15245: f64, t15259: f64, t15314: f64, t15321: f64, t15324: f64, t15327: f64, t15330: f64, t15349: f64, t9182: f64, t9192: f64, t9214: f64, t9221: f64) -> f64 {
    let t15351 = -t9182 + 0.91983333333333333333e-1_f64 * t9192 - t12093 + 0.18396666666666666667e0_f64 * t11850 - t9214 + 0.82785e-1_f64 * t15237 + 0.67094444444444444443e-1_f64 * t15239 - 0.20128333333333333333e0_f64 * t15241 - 0.10064166666666666667e0_f64 * t15243 - 0.11038e0_f64 * t15245 + t15314 + t12104 - 0.40256666666666666668e0_f64 * t11875 - t12115 + 0.13418888888888888889e0_f64 * t9221 + 0.36793333333333333333e-1_f64 * t11932 + 0.33547222222222222222e0_f64 * t15259 + 0.33114e0_f64 * t15321 + 0.49671e0_f64 * t15324 + 0.16557e0_f64 * t15327 - 0.5519e-1_f64 * t15330 + t15349;
    t15351
}
