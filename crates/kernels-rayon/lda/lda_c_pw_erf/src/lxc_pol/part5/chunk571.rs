//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 571/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk571(t128: f64, t2: f64, t3309: f64, t39: f64, t2715: f64, t103: f64, t1710: f64, t440: f64, t442: f64, t131: f64, t1125: f64, t120: f64, t133: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3310 = t128 * t2;
    let t3313 = 0.3264533333333333_f64 * t3309 * t3310 * t39;
    let t3318 = param_hyb_omega_0 * t2715;
    let t3319 = t103 * t2;
    let t3322 = 1.9486833333333333_f64 * t3318 * t3319 * t39;
    let t3332 = t440 * t1710;
    let t3337 = t442 * t442;
    let t3338 = 1.0_f64 / t3337;
    let t3339 = t131 * t3338;
    let t3348 = 0.8940581481481481_f64 * t133 * t1125 * t120;
    (t3310, t3313, t3318, t3319, t3322, t3332, t3337, t3338, t3339, t3348)
}
