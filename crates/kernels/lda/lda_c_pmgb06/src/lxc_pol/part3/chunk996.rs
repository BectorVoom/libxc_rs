//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 996/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk996<F: Float>(t12368: F, t4913: F, t5028: F, t405: F, t5025: F, t2057: F, t955: F, t2054: F, t103: F, t12156: F, t12161: F, t12165: F, t12364: F, t12366: F, t12371: F, t12374: F, t12377: F, t12380: F, t12382: F, t12384: F, t12387: F, t2060: F, t3404: F, t473: F, t9693: F, t9715: F, t9719: F) -> (F,) {
    let t13595 = 0.03199259259259259 * t12368;
    let t13602 = t4913 * t5028;
    let t13604 = t405 * t5025;
    let t13619 = t955 * t2057;
    let t13621 = t955 * t2054;
    let t13623 = 0.21595 * t12364 + 0.09597777777777777 * t12366 - t13595 - 0.023994444444444443 * t12371 - 0.14396666666666666 * t12374 - 0.10664197530864197 * t12377 - 0.23994444444444443 * t12380 + 0.07198333333333333 * t12384 + 0.4319 * t12387 + t9715 - 0.3466666666666667 * t13602 - 0.02666666666666667 * t13604 + 0.02666666666666667 * t9719 - 0.006913580246913581 * t103 * t9693 * t12156 - 0.017777777777777778 * t2060 * t3404 * t12161 + 0.013333333333333334 * t103 * t473 * t12382 + 0.08 * t2060 * t473 * t12165 + 0.044444444444444446 * t13619 - 0.007407407407407408 * t13621;
    (t13623,)
}
