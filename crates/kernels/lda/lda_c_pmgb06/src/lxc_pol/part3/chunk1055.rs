//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1055/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1055<F: Float>(t12546: F, t12547: F, t5068: F, t2956: F, t5077: F, t5078: F, t12514: F, t441: F, t5075: F, t5079: F, t1083: F, t4851: F) -> (F, F, F, F, F) {
    let t12550 = F::new(2.0) / F::new(5.0) * t5068 * t12546 * t12547;
    let t12553 = F::new(2.0) / F::new(15.0) * t5077 * t5078 * t2956;
    let t12555 = t5075 * t12514 * t441;
    let t12556 = t12555 * t5079;
    let t12557 = F::new(8.0) / F::new(45.0) * t12556;
    let t12558 = t4851 * t1083;
    (t12550, t12553, t12555, t12557, t12558)
}
