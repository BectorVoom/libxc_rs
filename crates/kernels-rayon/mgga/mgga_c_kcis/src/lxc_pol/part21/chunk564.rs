//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 564/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk564(t3246: f64, t3185: f64, t3193: f64, t3196: f64, t3207: f64, t3215: f64, t3222: f64, t3231: f64, t3235: f64, t3239: f64, t3242: f64, t3248: f64, t3319: f64, t3609: f64, t3623: f64, t3644: f64, t430: f64) -> (f64, f64) {
    let t3658 = 0.38691203703703703703e-3_f64 * t3246;
    let t3661 = 0.890445125e-2_f64 * t3644 * t3623 - 0.61905925925925925925e-2_f64 * t3185 + 0.11607361111111111111e-2_f64 * t3193 + 0.23214722222222222222e-2_f64 * t3196 - 0.23214722222222222222e-2_f64 * t3207 + 0.15476481481481481481e-2_f64 * t3215 + t3609 * t430 - 0.38691203703703703703e-3_f64 * t3222 + 0.34822083333333333332e-2_f64 * t3231 + 0.92858888888888888886e-2_f64 * t3235 + 0.17024129629629629629e-1_f64 * t3239 - 0.92858888888888888886e-2_f64 * t3242 - t3658 - 0.61905925925925925925e-2_f64 * t3248 - 0.17411041666666666666e-2_f64 * t3319;
    (t3658, t3661)
}
