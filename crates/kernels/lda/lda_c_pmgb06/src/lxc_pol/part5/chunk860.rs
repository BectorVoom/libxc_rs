//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 860/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk860<F: Float>(t247: F, t4344: F, t927: F, t101: F, t7245: F, t754: F, t757: F, t328: F, t113: F, t301: F, t395: F, t6716: F, t413: F, t5988: F, t5980: F, t76: F) -> (F, F, F, F, F, F) {
    let t14761 = t247 * t927 * t4344;
    let t14773 = t101 * t7245 * t754 * t757;
    let t14776 = t7245 * t328;
    let t14786 = t395 * t6716 * t113 * t301;
    let t14789 = t5988 * t413 * t301;
    let t14797 = t76 * t5980;
    (t14761, t14773, t14776, t14786, t14789, t14797)
}
