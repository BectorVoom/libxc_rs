//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 791/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk791<F: Float>(t1962: F, t464: F, t1386: F, t1988: F, t517: F, t1381: F, t2088: F, t497: F, t337: F, t1380: F, t3223: F, t835: F) -> (F, F, F, F, F, F, F, F) {
    let t5482 = t1962 * t464;
    let t5483 = t5482 * t1386;
    let t5486 = t1988 * t517;
    let t5487 = t5486 * t1381;
    let t5492 = t2088 * t497;
    let t5493 = t5492 * t337;
    let t5494 = t1380 * t5493;
    let t5497 = t3223 * t835;
    (t5482, t5483, t5486, t5487, t5492, t5493, t5494, t5497)
}
