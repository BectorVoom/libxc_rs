//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1047/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1047<F: Float>(t405: F, t6147: F, t4913: F, t6196: F, t6199: F, t6202: F, t103: F, t15353: F, t15360: F, t15365: F, t15369: F, t15395: F, t15440: F, t15445: F, t15589: F, t1619: F, t473: F, t9715: F) -> (F,) {
    let t15591 = t405 * t6147;
    let t15593 = t4913 * t6196;
    let t15601 = t405 * t6199;
    let t15603 = t405 * t6202;
    let t15611 = 0.14396666666666666 * t15360 - 0.047988888888888886 * t15365 - 0.023994444444444443 * t15369 + 0.05333333333333334 * t15589 - 0.017777777777777778 * t15591 - 0.2311111111111111 * t15593 - 0.04 * t103 * t473 * t15395 - 0.08 * t103 * t1619 * t15440 - 0.017777777777777778 * t15601 + 0.002962962962962963 * t15603 + 0.02666666666666667 * t103 * t473 * t15445 + 0.013333333333333334 * t103 * t473 * t15353 + t9715;
    (t15611,)
}
