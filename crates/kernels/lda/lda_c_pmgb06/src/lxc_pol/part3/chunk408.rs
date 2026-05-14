//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 408/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk408<F: Float>(t1560: F, t439: F, t1472: F, t147: F, t315: F, t146: F, t164: F, t405: F, t526: F, t162: F, t163: F) -> (F, F, F, F, F, F, F) {
    let t1562 = 2.0 / 45.0 * t439 * t1560;
    let t1563 = 0.047988888888888886 * t1472;
    let t1568 = t315 * t147;
    let t1571 = 0.011111111111111112 * t146 * t1568 * t164;
    let t1572 = t405 * t526;
    let t1575 = 1.0 / t163 / t162;
    let t1576 = t147 * t1575;
    (t1562, t1563, t1568, t1571, t1572, t1575, t1576)
}
