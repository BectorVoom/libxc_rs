//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk941<F: Float>(t1786: F, t27: F, t2767: F, t927: F, t2368: F, t754: F, t936: F, t97: F, t1789: F, t409: F, t328: F, t5915: F) -> (F, F, F, F) {
    let t10980 = t927 * t1786 * t27 * t2767;
    let t10984 = t2368 * t754 * t97 * t936;
    let t10985 = F::cast_from(0.41076328840066667_f64) * t10984;
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10991 = F::cast_from(1.898172889849454_f64) * t10990;
    let t10993 = t5915 * t328;
    (t10980, t10985, t10991, t10993)
}
