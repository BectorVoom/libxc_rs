//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 620/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk620<F: Float>(t1022: F, t3316: F, t1096: F, t1092: F, t300: F, t3038: F, t3180: F, t3185: F, t3193: F, t3196: F, t3207: F, t3215: F, t3222: F, t3231: F, t3235: F, t3239: F, t3242: F, t3247: F, t3248: F) -> (F, F, F, F) {
    let t3317 = t1022 * t3316;
    let t3318 = t1096 * t3317;
    let t3319 = t1092 * t3318;
    let t3322 = -F::new(0.49745833333333333332e-2) * t3180 - F::new(0.88437037037037037034e-2) * t3185 + F::new(0.16581944444444444444e-2) * t3193 + F::new(0.33163888888888888888e-2) * t3196 - F::new(0.33163888888888888888e-2) * t3207 + F::new(0.22109259259259259258e-2) * t3215 - F::new(0.55273148148148148147e-3) * t3222 + F::new(0.49745833333333333332e-2) * t3231 + F::new(0.13265555555555555555e-1) * t3235 + F::new(0.24320185185185185185e-1) * t3239 - F::new(0.13265555555555555555e-1) * t3242 - t3247 - F::new(0.88437037037037037034e-2) * t3248 - F::new(0.24872916666666666666e-2) * t3319 + t3038 * t300;
    (t3317, t3318, t3319, t3322)
}
