//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 658/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk658<F: Float>(t123: F, t290: F, t317: F, t4001: F, t342: F, t384: F, t374: F, t1227: F, t73: F, t1234: F, t113: F, t2778: F, t301: F) -> (F, F, F, F, F, F) {
    let t4005 = F::cast_from(0.9247854820715865_f64) * t123 * t4001 * t290 * t317;
    let t4006 = t384 * t342;
    let t4013 = t384 * t374;
    let t4017 = t73 * t1227;
    let t4021 = t73 * t1234;
    let t4027 = F::cast_from(0.006715335817467199_f64) * t2778 * t113 * t301;
    (t4005, t4006, t4013, t4017, t4021, t4027)
}
