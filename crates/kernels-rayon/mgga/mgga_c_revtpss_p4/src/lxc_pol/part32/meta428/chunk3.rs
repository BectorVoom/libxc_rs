//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1518/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1518(t12297: f64, t12367: f64, t16706: f64, t16820: f64, t16821: f64, t16822: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> f64 {
    let t20567 = -t12367 + 0.41203703703703703703e-2_f64 * t12297 + 0.82407407407407407408e-2_f64 * t16706 + t16820 - t16821 - t16822 + 0.20601851851851851852e-2_f64 * t20283 + 0.10300925925925925926e-1_f64 * t20295 - 0.37083333333333333333e-1_f64 * t20300 - 0.12361111111111111111e-1_f64 * t20304 - 0.61805555555555555557e-2_f64 * t20285 + 0.55625000000000000001e-1_f64 * t20308 + 0.37083333333333333334e-1_f64 * t20312 - 0.30902777777777777778e-2_f64 * t20287 - 0.61805555555555555555e-2_f64 * t20315 + 0.18541666666666666667e-1_f64 * t20320 + 0.92708333333333333333e-2_f64 * t20290;
    t20567
}
