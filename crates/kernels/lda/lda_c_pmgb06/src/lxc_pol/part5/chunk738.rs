//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk738<F: Float>(t36: F, t7494: F, t443: F, t7290: F, t453: F, t3081: F, t4635: F, t6205: F, t6207: F, t6209: F, t7479: F, t7483: F, t7487: F, t7491: F, t153: F, t137: F) -> (F, F, F, F, F, F, F) {
    let t7495 = t36 * t7494;
    let t7497 = t443 * t7290;
    let t7498 = t453 * t7497;
    let t7499 = t36 * t7498;
    let t7501 = t3081 + 0.002518888888888889 * t4635 - 0.0012594444444444445 * t6205 + 0.003778333333333333 * t6207 - 0.0018891666666666666 * t6209 + 0.002099074074074074 * t7479 - 0.007556666666666666 * t7483 + 0.003778333333333333 * t7487 + 0.011335 * t7491 - 0.011335 * t7495 + 0.0018891666666666666 * t7499;
    let t7502 = t7501 * t153;
    let t7503 = t137 * t7502;
    (t7495, t7497, t7498, t7499, t7501, t7502, t7503)
}
