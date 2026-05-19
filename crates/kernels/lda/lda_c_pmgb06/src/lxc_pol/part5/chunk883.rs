//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 883/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk883<F: Float>(t152: F, t3030: F, t134: F, t147: F, t1531: F, t2060: F, t474: F, t1147: F, t135: F, t146: F, t9177: F, t1697: F, t1730: F) -> (F, F, F, F, F, F, F) {
    let t9647 = F::new(1.0) / t3030 / t152;
    let t9693 = t147 / t134 / t1531;
    let t9702 = t2060 * t474;
    let t9712 = t1147 * t147;
    let t9715 = F::cast_from(0.10864197530864197_f64) * t146 * t9712 * t135;
    let t9724 = F::cast_from(0.3732469135802469_f64) * t9177;
    let t9759 = F::cast_from(0.19947266666666666_f64) * t1697 * t1730;
    (t9647, t9693, t9702, t9712, t9715, t9724, t9759)
}
