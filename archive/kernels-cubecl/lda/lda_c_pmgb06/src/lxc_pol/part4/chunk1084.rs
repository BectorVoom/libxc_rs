//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1084/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1084<F: Float>(t12535: F, t1461: F, t5065: F, t1464: F, t177: F, t12514: F, t441: F, t5075: F, t5079: F, t10220: F, t176: F, t10230: F) -> (F, F, F, F, F, F) {
    let t12537 = t5065 * t12535 * t1461;
    let t12546 = t177 * t1464;
    let t12555 = t5075 * t12514 * t441;
    let t12556 = t12555 * t5079;
    let t12580 = t10220 * t176;
    let t12592 = t10230 * t176;
    (t12537, t12546, t12555, t12556, t12580, t12592)
}
