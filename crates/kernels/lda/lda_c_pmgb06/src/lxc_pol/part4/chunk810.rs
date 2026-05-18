//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 810/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk810<F: Float>(t1381: F, t5486: F, t2088: F, t497: F, t337: F, t1380: F, t3223: F, t835: F, t160: F, t5210: F) -> (F, F, F, F, F) {
    let t5487 = t5486 * t1381;
    let t5492 = t2088 * t497;
    let t5493 = t5492 * t337;
    let t5494 = t1380 * t5493;
    let t5497 = t3223 * t835;
    let t5499 = t160 * t5210;
    (t5487, t5493, t5494, t5497, t5499)
}
