//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1017/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1017<F: Float>(t1623: F, t955: F, t1620: F, t134: F, t147: F, t1531: F, t1626: F, t2060: F, t474: F, t1147: F, t135: F, t146: F) -> (F, F, F, F, F, F, F) {
    let t9679 = t955 * t1623;
    let t9683 = t955 * t1620;
    let t9693 = t147 / t134 / t1531;
    let t9700 = t955 * t1626;
    let t9702 = t2060 * t474;
    let t9712 = t1147 * t147;
    let t9715 = F::cast_from(0.10864197530864197_f64) * t146 * t9712 * t135;
    (t9679, t9683, t9693, t9700, t9702, t9712, t9715)
}
