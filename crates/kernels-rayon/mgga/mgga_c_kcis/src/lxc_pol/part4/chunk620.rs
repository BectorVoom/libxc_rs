//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 620/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk620(t1022: f64, t3316: f64, t1096: f64, t1092: f64, t300: f64, t3038: f64, t3180: f64, t3185: f64, t3193: f64, t3196: f64, t3207: f64, t3215: f64, t3222: f64, t3231: f64, t3235: f64, t3239: f64, t3242: f64, t3247: f64, t3248: f64) -> (f64, f64, f64, f64) {
    let t3317 = t1022 * t3316;
    let t3318 = t1096 * t3317;
    let t3319 = t1092 * t3318;
    let t3322 = -0.49745833333333333332e-2_f64 * t3180 - 0.88437037037037037034e-2_f64 * t3185 + 0.16581944444444444444e-2_f64 * t3193 + 0.33163888888888888888e-2_f64 * t3196 - 0.33163888888888888888e-2_f64 * t3207 + 0.22109259259259259258e-2_f64 * t3215 - 0.55273148148148148147e-3_f64 * t3222 + 0.49745833333333333332e-2_f64 * t3231 + 0.13265555555555555555e-1_f64 * t3235 + 0.24320185185185185185e-1_f64 * t3239 - 0.13265555555555555555e-1_f64 * t3242 - t3247 - 0.88437037037037037034e-2_f64 * t3248 - 0.24872916666666666666e-2_f64 * t3319 + t3038 * t300;
    (t3317, t3318, t3319, t3322)
}
