//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 745/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk745(t422: f64, t3236: f64, t1124: f64, t1128: f64, t1127: f64, t432: f64, t427: f64, t3293: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3314 = t422 * t422;
    let t3315 = 1.0_f64 / t3314;
    let t3319 = 0.22831111111111111111e-1_f64 * t3236;
    let t3327 = t1124 * t1128;
    let t3330 = t1127 * t432;
    let t3331 = 1.0_f64 / t3330;
    let t3332 = t427 * t3331;
    let t3339 = 0.68863333333333333333e0_f64 * t3236;
    let t3346 = 0.17365833333333333333e0_f64 * t3293;
    let t3355 = t1127 * t1127;
    let t3356 = 1.0_f64 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358)
}
