//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 789/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk789<F: Float>(t1023: F, t1026: F, t138: F, t409: F, t109: F, t3674: F, t3676: F, t1767: F, t282: F, t55: F, t691: F, t1062: F, t3709: F, t696: F, t957: F, t962: F) -> (F, F, F, F, F) {
    let t8651 = 0.2849333333333333 * t138 * t409 * t1023 * t1026;
    let t8655 = 6.87343803774119 * t138 * t109 * t3674 * t3676;
    let t8659 = 0.0018989649058080863 * t691 * t55 * t1767 * t282;
    let t8668 = 623.3709278045327 * t696 * t3709 * t957 * t1062;
    let t8677 = t962 * t962;
    (t8651, t8655, t8659, t8668, t8677)
}
