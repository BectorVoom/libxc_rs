//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 696/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk696(t3348: f64, t4478: f64, t774: f64, t1248: f64, t4397: f64, t1213: f64, t1222: f64, t1244: f64, t3239: f64, t3241: f64, t3244: f64, t3268: f64, t3271: f64, t3340: f64, t3343: f64, t4402: f64, t4405: f64, t4409: f64, t4413: f64, t4419: f64, t4422: f64, t4425: f64, t4462: f64, t4466: f64, t4473: f64, t4476: f64) -> (f64, f64, f64) {
    let t4480 = t3348 * t774 * t4478;
    let t4484 = t1248 * t774 * t4397;
    let t4487 = t3239 + 7.0_f64 / 144.0_f64 * t3241 + 7.0_f64 / 144.0_f64 * t4402 + t3244 * t4405 / 16.0_f64 - t1213 * t4409 / 48.0_f64 + t4413 * t4419 / 1536.0_f64 + 7.0_f64 / 4608.0_f64 * t4422 + t3271 * t4425 / 768.0_f64 - t1222 * t4462 / 3072.0_f64 - t3271 * t4466 / 3072.0_f64 + 7.0_f64 / 4608.0_f64 * t3268 + t3340 + 7.0_f64 / 1152.0_f64 * t3343 + t3271 * t4473 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t4476 + 5.0_f64 / 768.0_f64 * t1244 * t4480 - t1244 * t4484 / 768.0_f64;
    (t4480, t4484, t4487)
}
