//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1026/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1026(t12247: f64, t408: f64, t12228: f64, t3435: f64, t3418: f64, t698: f64, t240: f64, t3698: f64, t3361: f64, t635: f64, t10356: f64, t141: f64) -> (f64, f64, f64, f64, f64) {
    let t12248 = t408 * t12247;
    let t12249 = t12228 * t3435;
    let t12251 = 0.96491876992155210402e2_f64 * t12248 * t12249;
    let t12252 = t698 * t3418;
    let t12254 = t240 * t3698;
    let t12256 = 1.0_f64 / t3361 / t635;
    let t12257 = t12256 * t10356;
    let t12258 = t12254 * t12257;
    let t12259 = t141 * t12258;
    (t12251, t12252, t12256, t12257, t12259)
}
