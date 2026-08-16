//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 784/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk784(t520: f64, t5407: f64, t1224: f64, t774: f64, t5380: f64, t3348: f64, t5371: f64, t1248: f64, t5366: f64, t1213: f64, t1222: f64, t1244: f64, t3239: f64, t3244: f64, t3258: f64, t3271: f64, t3340: f64, t4402: f64, t4422: f64, t4476: f64, t5373: f64, t5377: f64, t5383: f64, t5389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5408 = t5407 * t520;
    let t5410 = t1224 * t774 * t5408;
    let t5413 = t5380 * t520;
    let t5415 = t1224 * t774 * t5413;
    let t5420 = t3348 * t774 * t5371;
    let t5424 = t1248 * t774 * t5366;
    let t5427 = t3239 + 7.0_f64 / 72.0_f64 * t4402 + t3244 * t5373 / 16.0_f64 - t1213 * t5377 / 48.0_f64 + t3258 * t5383 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t4422 + t3271 * t5389 / 384.0_f64 - t1222 * t5410 / 3072.0_f64 - t1222 * t5415 / 3072.0_f64 + t3340 + 7.0_f64 / 576.0_f64 * t4476 + 5.0_f64 / 768.0_f64 * t1244 * t5420 - t1244 * t5424 / 768.0_f64;
    (t5408, t5410, t5413, t5415, t5420, t5424, t5427)
}
