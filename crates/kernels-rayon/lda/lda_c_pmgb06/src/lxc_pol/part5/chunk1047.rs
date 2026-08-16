//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1047/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1047(t19523: f64, t1447: f64, t7513: f64, t15829: f64, t15831: f64, t15835: f64, t15838: f64, t11898: f64, t19514: f64, t19518: f64, t19521: f64, t19522: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19524 = 2.0_f64 / 45.0_f64 * t19523;
    let t19525 = t1447 * t7513;
    let t19526 = 4.0_f64 / 45.0_f64 * t19525;
    let t19527 = 8.0_f64 / 45.0_f64 * t15829;
    let t19528 = 4.0_f64 / 45.0_f64 * t15831;
    let t19529 = 4.0_f64 / 45.0_f64 * t15835;
    let t19530 = 8.0_f64 / 45.0_f64 * t15838;
    let t19531 = t19514 + t19518 + t19521 + t11898 - t19522 - t19524 - t19526 - t19527 + t19528 + t19529 - t19530;
    (t19524, t19526, t19527, t19528, t19529, t19530, t19531)
}
