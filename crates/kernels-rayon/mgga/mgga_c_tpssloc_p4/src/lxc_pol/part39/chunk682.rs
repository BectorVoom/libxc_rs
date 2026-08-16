//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 682/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk682(t1137: f64, t3351: f64, t1127: f64, t427: f64, t435: f64, t3333: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t449: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3352 = t3351 * t1137;
    let t3355 = t1127 * t1127;
    let t3356 = 1.0_f64 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    let t3359 = 1.0_f64 / t3358;
    let t3360 = t3333 * t3359;
    let t3363 = 0.12361111111111111111e-1_f64 * t3236;
    let t3368 = t3363 - 0.61805555555555555556e-2_f64 * t3238 - 0.61805555555555555555e-2_f64 * t3245 + 0.18541666666666666667e-1_f64 * t3250 + 0.92708333333333333333e-2_f64 * t3254;
    let t3369 = t3368 * t449;
    (t3352, t3355, t3356, t3357, t3358, t3359, t3360, t3363, t3368, t3369)
}
