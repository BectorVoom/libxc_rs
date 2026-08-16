//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1514/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1514(t1150: f64, t20447: f64, t1131: f64, t12243: f64, t6474: f64, t3531: f64, t6548: f64, t12297: f64, t12382: f64, t16706: f64, t16708: f64, t16797: f64, t16798: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64, f64, f64) {
    let t20448 = t20447 * t1150;
    let t20450 = 1.0_f64 * t1131 * t20448;
    let t20452 = 0.16081979498692535067e2_f64 * t12243 * t6474;
    let t20454 = 0.11696447245269292414e1_f64 * t3531 * t6548;
    let t20469 = -t12382 + 0.79148148148148148147e-2_f64 * t12297 + 0.15829629629629629629e-1_f64 * t16706 + 0.79148148148148148147e-2_f64 * t16708 - t16797 - t16798 + 0.39574074074074074073e-2_f64 * t20283 + 0.19787037037037037037e-1_f64 * t20295 - 0.71233333333333333332e-1_f64 * t20300 - 0.23744444444444444444e-1_f64 * t20304 - 0.11872222222222222222e-1_f64 * t20285 + 0.10685e0_f64 * t20308 + 0.71233333333333333332e-1_f64 * t20312 - 0.5936111111111111111e-2_f64 * t20287 - 0.11872222222222222222e-1_f64 * t20315 + 0.35616666666666666666e-1_f64 * t20320 + 0.17808333333333333333e-1_f64 * t20290;
    (t20450, t20452, t20454, t20469)
}
