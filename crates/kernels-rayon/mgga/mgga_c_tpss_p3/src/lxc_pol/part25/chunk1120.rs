//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1120/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1120(t11845: f64, t11850: f64, t11873: f64, t11876: f64, t11911: f64, t11932: f64, t15237: f64, t15239: f64, t15241: f64, t15243: f64, t15245: f64, t15259: f64, t15321: f64, t15324: f64, t15327: f64, t15330: f64, t15385: f64, t15406: f64, t9192: f64, t9221: f64, t9297: f64, t9306: f64) -> f64 {
    let t15408 = -t9297 + 0.91285185185185185187e-1_f64 * t9192 - t11845 + 0.18257037037037037037e0_f64 * t11850 - t9306 + 0.82156666666666666667e-1_f64 * t15237 + 0.66437037037037037037e-1_f64 * t15239 - 0.19931111111111111111e0_f64 * t15241 - 0.99655555555555555557e-1_f64 * t15243 - 0.10954222222222222222e0_f64 * t15245 + t15385 + 0.13287407407407407407e0_f64 * t11873 - t11876 - t11911 + 0.13287407407407407408e0_f64 * t9221 + 0.36514074074074074073e-1_f64 * t11932 + 0.33218518518518518518e0_f64 * t15259 + 0.32862666666666666666e0_f64 * t15321 + 0.49293999999999999999e0_f64 * t15324 + 0.16431333333333333333e0_f64 * t15327 - 0.54771111111111111112e-1_f64 * t15330 + t15406;
    t15408
}
