//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 505/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk505(t1127: f64, t427: f64, t435: f64, t3236: f64, t1143: f64, t1147: f64, t1146: f64, t445: f64, t440: f64, t3293: f64, t448: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3355 = t1127 * t1127;
    let t3356 = 1.0_f64 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    let t3359 = 1.0_f64 / t3358;
    let t3363 = 0.12361111111111111111e-1_f64 * t3236;
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    let t3375 = 1.0_f64 / t3374;
    let t3376 = t440 * t3375;
    let t3383 = 0.40256666666666666667e0_f64 * t3236;
    let t3390 = 0.137975e0_f64 * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = 1.0_f64 / t3399;
    let t3401 = t440 * t3400;
    let t3402 = t448 * t448;
    let t3403 = 1.0_f64 / t3402;
    let t3411 = t300 * t1143;
    (t3357, t3359, t3363, t3371, t3375, t3376, t3383, t3390, t3400, t3401, t3403, t3411)
}
