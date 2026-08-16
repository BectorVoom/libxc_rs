//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 786/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk786(t6457: f64, t6459: f64, t6463: f64, t6467: f64, t6471: f64, t6475: f64, t6477: f64, t6481: f64, t6485: f64, t6487: f64, t6491: f64, t6495: f64, t6570: f64, t6572: f64, t6574: f64, t6576: f64) -> f64 {
    let t7254 = t6457 - t6459 - t6463 - t6467 - t6471 + t6475 + t6477 - t6481 - t6485 - t6487 + t6491 + t6495 - t6570 + t6572 - t6574 - t6576;
    t7254
}
