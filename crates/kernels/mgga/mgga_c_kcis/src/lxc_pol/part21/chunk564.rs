//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 564/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk564<F: Float>(t3246: F, t3185: F, t3193: F, t3196: F, t3207: F, t3215: F, t3222: F, t3231: F, t3235: F, t3239: F, t3242: F, t3248: F, t3319: F, t3609: F, t3623: F, t3644: F, t430: F) -> (F, F) {
    let t3658 = F::new(0.38691203703703703703e-3) * t3246;
    let t3661 = F::new(0.890445125e-2) * t3644 * t3623 - F::new(0.61905925925925925925e-2) * t3185 + F::new(0.11607361111111111111e-2) * t3193 + F::new(0.23214722222222222222e-2) * t3196 - F::new(0.23214722222222222222e-2) * t3207 + F::new(0.15476481481481481481e-2) * t3215 + t3609 * t430 - F::new(0.38691203703703703703e-3) * t3222 + F::new(0.34822083333333333332e-2) * t3231 + F::new(0.92858888888888888886e-2) * t3235 + F::new(0.17024129629629629629e-1) * t3239 - F::new(0.92858888888888888886e-2) * t3242 - t3658 - F::new(0.61905925925925925925e-2) * t3248 - F::new(0.17411041666666666666e-2) * t3319;
    (t3658, t3661)
}
