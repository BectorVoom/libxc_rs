//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1245/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1245(t1325: f64, t3787: f64, t7576: f64, t2120: f64, t6220: f64, t6209: f64, t18138: f64, t21577: f64, t2504: f64, t266: f64, t514: f64, t548: f64, t7837: f64) -> (f64, f64, f64, f64, f64) {
    let t22349 = t1325 * t3787 * t7576;
    let t22350 = 16.0_f64 / 15.0_f64 * t22349;
    let t22351 = t2120 * t6220;
    let t22352 = 8.0_f64 / 15.0_f64 * t22351;
    let t22353 = t6209 * t6220;
    let t22354 = 8.0_f64 / 15.0_f64 * t22353;
    let t22358 = 4.0_f64 / 5.0_f64 * t21577 * t18138 * t266 * t2504;
    let t22360 = t548 * t514 * t7837;
    (t22350, t22352, t22354, t22358, t22360)
}
