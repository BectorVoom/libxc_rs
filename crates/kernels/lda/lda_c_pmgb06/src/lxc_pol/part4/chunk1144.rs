//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1144/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1144<F: Float>(t1080: F, t2389: F, t2918: F, t1476: F, t36: F, t15200: F, t506: F, t103: F, t16905: F, t16910: F, t17162: F, t17164: F, t17166: F, t17169: F, t17172: F, t17175: F, t17177: F, t17185: F, t3358: F, t9967: F) -> (F, F, F, F) {
    let t17188 = t2918 * t2389 * t1080;
    let t17190 = t36 * t1476 * t17188;
    let t17193 = t36 * t506 * t15200;
    let t17195 = 0.07198333333333333 * t17162 + 0.026660493827160493 * t17164 - 0.3519185185185185 * t17166 - 0.03999074074074074 * t17169 - 0.10664197530864197 * t17172 + 0.14396666666666666 * t17175 + 0.14396666666666666 * t17177 - 0.002962962962962963 * t103 * t3358 * t16905 - 0.006913580246913581 * t103 * t9967 * t16910 - 0.017777777777777778 * t17185 + 0.14396666666666666 * t17190 - 0.21595 * t17193;
    (t17188, t17190, t17193, t17195)
}
