//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 491/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk491(t4072: f64, t4105: f64, t1360: f64, t1404: f64, t1455: f64, t3951: f64, t4018: f64, t4019: f64, t4021: f64, t4023: f64, t4024: f64, t4028: f64, t4031: f64, t4036: f64, t4039: f64, t486: f64, t510: f64, t538: f64) -> (f64, f64) {
    let t4106 = t4072 + t4105;
    let t4108 = t4018 + 0.46853067927761790996e-2_f64 * t4019 + 0.93706135855523581992e-2_f64 * t4021 + 0.46853067927761790996e-2_f64 * t4023 * t4024 + 0.93706135855523581992e-2_f64 * t1404 * t4028 - 0.23426533963880895498e-2_f64 * t1404 * t4031 + 0.14055920378328537299e-1_f64 * t510 * t4036 - 0.46853067927761790996e-2_f64 * t510 * t4039 - t3951 * t538 - 2.0_f64 * t1360 * t1455 - t486 * t4106;
    (t4106, t4108)
}
