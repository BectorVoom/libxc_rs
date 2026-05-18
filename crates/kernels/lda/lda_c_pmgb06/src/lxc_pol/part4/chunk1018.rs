//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1018/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1018<F: Float>(t9177: F, t1697: F, t1730: F, t2852: F, t432: F, t132: F, t2851: F, t459: F, t1179: F, t136: F, t154: F, t1554: F, t1587: F, t161: F) -> (F, F, F, F, F, F) {
    let t9724 = F::new(0.3732469135802469) * t9177;
    let t9759 = F::new(0.19947266666666666) * t1697 * t1730;
    let t9762 = t432 * t2852;
    let t9765 = t132 * t2851 * t459;
    let t9770 = F::new(28.0) / F::new(1215.0) * t132 * t1179 * t136 * t154;
    let t9774 = t161 * t1554 * t1587;
    (t9724, t9759, t9762, t9765, t9770, t9774)
}
