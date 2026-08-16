//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 785/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk785(t6395: f64, t6399: f64, t6403: f64, t6407: f64, t6411: f64, t6416: f64, t6421: f64, t6425: f64, t6430: f64, t6435: f64, t6437: f64, t6439: f64, t6441: f64, t6445: f64, t6449: f64, t6451: f64, t6453: f64) -> f64 {
    let t7252 = -t6395 - t6399 + t6403 - t6407 + t6411 + t6416 + t6421 + t6425 + t6430 + t6435 - t6437 - t6439 + t6441 - t6445 + t6449 + t6451 + t6453;
    t7252
}
