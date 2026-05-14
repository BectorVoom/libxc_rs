//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1029/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1029<F: Float>(t517: F, t7616: F, t161: F, t489: F, t7858: F, t166: F, t17919: F, t17921: F, t17926: F, t17931: F, t17935: F, t17938: F, t17960: F, t2088: F, t21117: F, t21139: F, t21184: F, t21218: F, t518: F, t529: F, t6230: F, t6736: F, t802: F) -> (F,) {
    let t21230 = t7616 * t517;
    let t21237 = t161 * t489 * t7858;
    let t21240 = 2.0 / 15.0 * t17919 + 2.0 / 15.0 * t17921 - t161 * t166 * t6230 * t2088 / 10.0 - t161 * t166 * t518 * (t21117 + t21139 + t21184 + t21218) / 30.0 - t802 * t6736 / 10.0 + t17926 / 45.0 + 2.0 / 45.0 * t17931 + 2.0 / 45.0 * t17935 - t161 * t166 * t21230 * t529 / 30.0 - 2.0 / 15.0 * t17938 - t21237 / 45.0 - t17960 / 15.0;
    (t21240,)
}
