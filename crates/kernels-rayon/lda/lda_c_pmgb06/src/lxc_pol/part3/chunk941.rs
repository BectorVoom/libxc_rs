//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk941(t1786: f64, t27: f64, t2767: f64, t927: f64, t2368: f64, t754: f64, t936: f64, t97: f64, t1789: f64, t409: f64, t328: f64, t5915: f64) -> (f64, f64, f64, f64) {
    let t10980 = t927 * t1786 * t27 * t2767;
    let t10984 = t2368 * t754 * t97 * t936;
    let t10985 = 0.41076328840066667_f64 * t10984;
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10991 = 1.898172889849454_f64 * t10990;
    let t10993 = t5915 * t328;
    (t10980, t10985, t10991, t10993)
}
