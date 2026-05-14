//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 807/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk807<F: Float>(t132: F, t2851: F, t459: F, t1179: F, t136: F, t154: F, t161: F, t3004: F, t530: F, t3457: F, t496: F, t2060: F, t526: F, t1414: F, t147: F, t163: F) -> (F, F, F, F, F, F) {
    let t9765 = t132 * t2851 * t459;
    let t9770 = 28.0 / 1215.0 * t132 * t1179 * t136 * t154;
    let t9890 = t161 * t3004 * t530;
    let t9908 = t496 * t3457;
    let t9938 = t2060 * t526;
    let t9967 = t147 / t163 / t1414;
    (t9765, t9770, t9890, t9908, t9938, t9967)
}
