//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 626/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk626<F: Float>(t2181: F, t2209: F, t24: F, t5582: F, t4042: F, t73: F, t374: F, t783: F, t342: F, t4232: F, t1233: F) -> (F, F, F, F, F, F, F, F) {
    let t5999 = t2181 * t2209;
    let t6006 = t24 * t5582;
    let t6007 = t4042 * t73;
    let t6008 = t783 * t374;
    let t6009 = t6007 * t6008;
    let t6012 = t783 * t342;
    let t6013 = t4232 * t6012;
    let t6018 = t1233 * t5582;
    (t5999, t6006, t6007, t6008, t6009, t6012, t6013, t6018)
}
