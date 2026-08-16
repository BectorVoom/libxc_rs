//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 696/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk696(t1250: f64, t3342: f64, t508: f64, t526: f64, t235: f64, t72: f64, t3245: f64, t774: f64, t1248: f64, t3234: f64, t1213: f64, t1222: f64, t1244: f64, t3239: f64, t3241: f64, t3244: f64, t3247: f64, t3251: f64, t3258: f64, t3263: f64, t3268: f64, t3271: f64, t3277: f64, t3329: f64, t3334: f64, t3340: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3343 = t3342 * t1250;
    let t3346 = 1.0_f64 / t526 / t508;
    let t3347 = t235 * t3346;
    let t3348 = t3347 * t72;
    let t3350 = t3348 * t774 * t3245;
    let t3354 = t1248 * t774 * t3234;
    let t3357 = t3239 + 7.0_f64 / 72.0_f64 * t3241 + t3244 * t3247 / 16.0_f64 - t1213 * t3251 / 48.0_f64 + t3258 * t3263 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t3268 + t3271 * t3277 / 384.0_f64 - t1222 * t3329 / 3072.0_f64 - t1222 * t3334 / 3072.0_f64 + t3340 + 7.0_f64 / 576.0_f64 * t3343 + 5.0_f64 / 768.0_f64 * t1244 * t3350 - t1244 * t3354 / 768.0_f64;
    (t3343, t3346, t3348, t3350, t3354, t3357)
}
