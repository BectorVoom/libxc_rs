//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1241/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1241(t3057: f64, t3286: f64, t1071: f64, t1086: f64, t994: f64, t3316: f64, t989: f64, t11239: f64, t11627: f64, t342: f64, t1129: f64, t3431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12149 = t3057 * t3286;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12160 = t989 * t3316;
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12226 = 1.0_f64 / t3431 / t1129;
    (t12149, t12154, t12160, t12166, t12167, t12226)
}
