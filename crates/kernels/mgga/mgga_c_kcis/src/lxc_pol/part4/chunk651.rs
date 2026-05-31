//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 651/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk651<F: Float>(t1238: F, t429: F, t413: F, t3246: F, t3185: F, t3193: F, t3196: F, t3207: F, t3215: F, t3222: F, t3231: F, t3235: F, t3239: F, t3242: F, t3248: F, t3319: F, t3609: F, t3623: F, t430: F) -> (F, F, F, F) {
    let t3643 = F::cast_from(1.0_f64) / t1238 / t429;
    let t3644 = t413 * t3643;
    let t3658 = F::cast_from(0.38691203703703703703e-3_f64) * t3246;
    let t3661 = F::cast_from(0.890445125e-2_f64) * t3644 * t3623 - F::cast_from(0.61905925925925925925e-2_f64) * t3185 + F::cast_from(0.11607361111111111111e-2_f64) * t3193 + F::cast_from(0.23214722222222222222e-2_f64) * t3196 - F::cast_from(0.23214722222222222222e-2_f64) * t3207 + F::cast_from(0.15476481481481481481e-2_f64) * t3215 + t3609 * t430 - F::cast_from(0.38691203703703703703e-3_f64) * t3222 + F::cast_from(0.34822083333333333332e-2_f64) * t3231 + F::cast_from(0.92858888888888888886e-2_f64) * t3235 + F::cast_from(0.17024129629629629629e-1_f64) * t3239 - F::cast_from(0.92858888888888888886e-2_f64) * t3242 - t3658 - F::cast_from(0.61905925925925925925e-2_f64) * t3248 - F::cast_from(0.17411041666666666666e-2_f64) * t3319;
    (t3643, t3644, t3658, t3661)
}
