//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 680/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk680(t3439: f64, t3442: f64, t3444: f64, t3447: f64, t3449: f64, t3451: f64, t3453: f64, t3457: f64, t3459: f64, t3461: f64, t3463: f64, t3468: f64, t3549: f64, t3552: f64, t3555: f64, t3558: f64, t3560: f64) -> f64 {
    let t4177 = t3439 + t3442 + t3444 + t3447 - t3449 + t3451 - t3453 + t3457 - t3459 - t3461 + t3463 + t3468 - t3549 - t3552 + t3555 - t3558 - t3560;
    t4177
}
