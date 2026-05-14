//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 618/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk618<F: Float>(t1121: F, t3960: F, t110: F, t959: F, t968: F, t30: F, t653: F) -> (F, F, F, F, F, F) {
    let t3962 = 0.02168716260060348 * t1121 * t3960;
    let t3963 = t110 * t959;
    let t3965 = 0.01626537195045261 * t1121 * t3963;
    let t3966 = t110 * t968;
    let t3968 = 0.4815973313767657 * t1121 * t3966;
    let t3969 = t653 * t30;
    (t3962, t3963, t3965, t3966, t3968, t3969)
}
