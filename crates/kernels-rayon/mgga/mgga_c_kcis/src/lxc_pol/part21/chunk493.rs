//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 493/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk493(t3135: f64, t3165: f64, t1030: f64, t1083: f64, t278: f64, t3038: f64, t305: f64, t3056: f64, t3057: f64, t3059: f64, t3061: f64, t3062: f64, t3066: f64, t3069: f64, t3075: f64, t3097: f64, t339: f64, t975: f64) -> (f64, f64) {
    let t3166 = t3135 + t3165;
    let t3168 = t3056 + 0.46853067927761790996e-2_f64 * t3057 + 0.93706135855523581992e-2_f64 * t3059 + 0.46853067927761790996e-2_f64 * t3061 * t3062 + 0.93706135855523581992e-2_f64 * t1030 * t3066 - 0.23426533963880895498e-2_f64 * t1030 * t3069 + 0.14055920378328537299e-1_f64 * t305 * t3075 - 0.46853067927761790996e-2_f64 * t305 * t3097 - t3038 * t339 - 2.0_f64 * t975 * t1083 - t278 * t3166;
    (t3166, t3168)
}
