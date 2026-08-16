//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 938/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk938(t15860: f64, t5909: f64, t4260: f64, t12265: f64, t4291: f64, t6012: f64, t17331: f64, t17335: f64, t17337: f64, t17339: f64, t17342: f64, t17344: f64, t17347: f64, t17350: f64, t17353: f64, t17355: f64, t17358: f64, t17360: f64, t17362: f64, t17364: f64, t17366: f64, t17368: f64) -> (f64, f64, f64, f64) {
    let t17370 = t5909 * t15860;
    let t17371 = t4260 * t17370;
    let t17373 = t12265 * t4291;
    let t17374 = t17373 * t6012;
    let t17376 = -t17331 / 256.0_f64 - t17335 / 48.0_f64 + t17337 / 24.0_f64 - 2.0_f64 / 9.0_f64 * t17339 - t17342 / 576.0_f64 - t17344 / 8.0_f64 - t17347 / 36.0_f64 + t17350 / 576.0_f64 + t17353 / 24.0_f64 - t17355 / 16.0_f64 + t17358 / 4.0_f64 - t17360 / 16.0_f64 + t17362 / 48.0_f64 - t17364 / 12.0_f64 + t17366 / 96.0_f64 - t17368 / 576.0_f64 + t17371 / 72.0_f64 - t17374 / 64.0_f64;
    (t17370, t17371, t17374, t17376)
}
