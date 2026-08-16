//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1255/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1255(t11239: f64, t1243: f64, t460: f64, t3596: f64, t13038: f64, t1275: f64, t225: f64, t10270: f64, t10272: f64, t10279: f64, t10281: f64, t10288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0_f64 / t13180;
    let t13182 = t225 * t13181;
    let t13261 = 4.0_f64 * t10270;
    let t13262 = 12.0_f64 * t10272;
    let t13263 = 48.0_f64 * t10279;
    let t13264 = 80.0_f64 * t10281;
    let t13265 = 180.0_f64 * t10288;
    (t13126, t13127, t13141, t13142, t13147, t13148, t13182, t13261, t13262, t13263, t13264, t13265)
}
