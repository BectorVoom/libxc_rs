//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 533/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk533(t3423: f64, t355: f64, t377: f64, t3217: f64, t3219: f64, t376: f64, t375: f64, t1188: f64, t1195: f64, t1187: f64, t3335: f64, t3340: f64, t3344: f64, t3349: f64, t3356: f64, t3359: f64, t3363: f64, t3366: f64, t3370: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3424 = t3423 * t355;
    let t3425 = t3424 * sigma0;
    let t3426 = t3425 * t377;
    let t3428 = t3217 * t3219;
    let t3429 = t376 * t3428;
    let t3430 = t375 * t3429;
    let t3432 = t1195 * t1188;
    let t3433 = t1187 * t3432;
    let t3435 = t3335 / 3.0_f64 - t3340 / 12.0_f64 + t3344 / 24.0_f64 - t3349 / 576.0_f64 - 19.0_f64 / 144.0_f64 * t3356 + t3359 / 18.0_f64 + 11.0_f64 / 18.0_f64 * t3363 - 2.0_f64 / 9.0_f64 * t3366 - t3370 / 256.0_f64 + t3426 / 16.0_f64 - t3430 / 72.0_f64 - t3433 / 24.0_f64;
    (t3424, t3425, t3426, t3429, t3430, t3432, t3433, t3435)
}
