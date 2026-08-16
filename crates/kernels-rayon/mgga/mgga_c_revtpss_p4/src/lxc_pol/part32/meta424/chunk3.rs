//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1506/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1506(t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t12296: f64, t12297: f64, t16706: f64, t16915: f64, t16916: f64, t16917: f64) -> (f64, f64) {
    let t20322 = 0.67094444444444444443e-1_f64 * t20283 - 0.20128333333333333333e0_f64 * t20285 - 0.10064166666666666667e0_f64 * t20287 + 0.301925e0_f64 * t20290 + 0.33547222222222222222e0_f64 * t20295 - 0.12077e1_f64 * t20300 - 0.40256666666666666666e0_f64 * t20304 + 0.181155e1_f64 * t20308 + 0.12077e1_f64 * t20312 - 0.20128333333333333333e0_f64 * t20315 + 0.60385e0_f64 * t20320;
    let t20337 = -t12296 + 4.0_f64 / 27.0_f64 * t12297 + 8.0_f64 / 27.0_f64 * t16706 + t16915 - t16916 - t16917 + 2.0_f64 / 27.0_f64 * t20283 + 10.0_f64 / 27.0_f64 * t20295 - 4.0_f64 / 3.0_f64 * t20300 - 4.0_f64 / 9.0_f64 * t20304 - 2.0_f64 / 9.0_f64 * t20285 + 2.0_f64 * t20308 + 4.0_f64 / 3.0_f64 * t20312 - t20287 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t20315 + 2.0_f64 / 3.0_f64 * t20320 + t20290 / 3.0_f64;
    (t20322, t20337)
}
