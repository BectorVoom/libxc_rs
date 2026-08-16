//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 437/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk437(t214: f64, t3139: f64, t3138: f64, t1001: f64, t1050: f64, t982: f64, t3174: f64, t117: f64, t212: f64, t211: f64, t210: f64, t3234: f64, t3237: f64, t3239: f64, t3243: f64, t3246: f64, t3249: f64, t3251: f64, t3254: f64, t3256: f64, t3258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3260 = t214 * t3139;
    let t3261 = t3138 * t3260;
    let t3263 = t1050 * t1001;
    let t3264 = t982 * t3263;
    let t3266 = t214 * t3174;
    let t3267 = t982 * t3266;
    let t3269 = t212 * t117;
    let t3270 = 1.0_f64 / t3269;
    let t3271 = t211 * t3270;
    let t3272 = t210 * t3271;
    let t3274 = t3234 / 8.0_f64 - t3237 / 4.0_f64 - t3239 / 2.0_f64 + t3243 / 4.0_f64 + t3246 / 2.0_f64 - t3249 / 8.0_f64 + 3.0_f64 / 4.0_f64 * t3251 - t3254 / 64.0_f64 + t3256 / 32.0_f64 + t3258 / 8.0_f64 - t3261 / 32.0_f64 - t3264 / 8.0_f64 + t3267 / 64.0_f64 - 5.0_f64 / 16.0_f64 * t3272;
    (t3260, t3261, t3263, t3264, t3266, t3267, t3271, t3272, t3274)
}
