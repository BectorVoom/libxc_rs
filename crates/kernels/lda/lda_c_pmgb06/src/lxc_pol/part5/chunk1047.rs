//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1047/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1047<F: Float>(t19523: F, t1447: F, t7513: F, t15829: F, t15831: F, t15835: F, t15838: F, t11898: F, t19514: F, t19518: F, t19521: F, t19522: F) -> (F, F, F, F, F, F, F) {
    let t19524 = F::new(2.0) / F::new(45.0) * t19523;
    let t19525 = t1447 * t7513;
    let t19526 = F::new(4.0) / F::new(45.0) * t19525;
    let t19527 = F::new(8.0) / F::new(45.0) * t15829;
    let t19528 = F::new(4.0) / F::new(45.0) * t15831;
    let t19529 = F::new(4.0) / F::new(45.0) * t15835;
    let t19530 = F::new(8.0) / F::new(45.0) * t15838;
    let t19531 = t19514 + t19518 + t19521 + t11898 - t19522 - t19524 - t19526 - t19527 + t19528 + t19529 - t19530;
    (t19524, t19526, t19527, t19528, t19529, t19530, t19531)
}
