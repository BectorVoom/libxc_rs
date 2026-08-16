//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1344/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1344(t12297: f64, t12678: f64, t16706: f64, t17319: f64, t17320: f64, t17321: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> f64 {
    let t21332 = -t12678 + 0.37037037037037037037e-2_f64 * t12297 + 0.74074074074074074074e-2_f64 * t16706 + t17319 - t17320 - t17321 + 0.18518518518518518518e-2_f64 * t20283 + 0.92592592592592592592e-2_f64 * t20295 - 0.33333333333333333333e-1_f64 * t20300 - 0.11111111111111111111e-1_f64 * t20304 - 0.55555555555555555557e-2_f64 * t20285 + 0.50000000000000000001e-1_f64 * t20308 + 0.33333333333333333334e-1_f64 * t20312 - 0.27777777777777777778e-2_f64 * t20287 - 0.55555555555555555555e-2_f64 * t20315 + 0.16666666666666666667e-1_f64 * t20320 + 0.83333333333333333333e-2_f64 * t20290;
    t21332
}
