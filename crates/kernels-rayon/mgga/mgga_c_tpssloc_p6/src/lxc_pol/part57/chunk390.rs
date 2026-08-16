//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 390/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk390(t3293: f64, t1176: f64, t241: f64, t1097: f64, t409: f64, t422: f64, t3236: f64, t1127: f64, t432: f64, t427: f64, t435: f64, t1146: f64, t445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3294 = 0.13692777777777777778e0_f64 * t3293;
    let t3297 = t241 * t1176;
    let t3311 = t1097 * t1097;
    let t3312 = 1.0_f64 / t3311;
    let t3313 = t409 * t3312;
    let t3314 = t422 * t422;
    let t3315 = 1.0_f64 / t3314;
    let t3319 = 0.22831111111111111111e-1_f64 * t3236;
    let t3330 = t1127 * t432;
    let t3331 = 1.0_f64 / t3330;
    let t3332 = t427 * t3331;
    let t3339 = 0.68863333333333333333e0_f64 * t3236;
    let t3346 = 0.17365833333333333333e0_f64 * t3293;
    let t3355 = t1127 * t1127;
    let t3356 = 1.0_f64 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    let t3359 = 1.0_f64 / t3358;
    let t3363 = 0.12361111111111111111e-1_f64 * t3236;
    let t3374 = t1146 * t445;
    (t3294, t3297, t3313, t3315, t3319, t3332, t3339, t3346, t3357, t3359, t3363, t3374)
}
