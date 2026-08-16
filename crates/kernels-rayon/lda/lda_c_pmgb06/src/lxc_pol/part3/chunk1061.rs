//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1061/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1061(t12574: f64, t12576: f64, t12579: f64, t12583: f64, t12587: f64, t12591: f64, t12597: f64, t12602: f64, t12604: f64, t12608: f64, t12610: f64, t12612: f64) -> f64 {
    let t12613 = t12574 + t12576 + t12579 + t12583 + t12587 - t12591 + t12597 - t12602 - t12604 - t12608 + t12610 - t12612;
    t12613
}
