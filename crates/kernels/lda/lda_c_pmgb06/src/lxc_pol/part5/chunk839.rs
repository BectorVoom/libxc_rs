//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 839/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk839<F: Float>(t2852: F, t802: F, t1554: F, t161: F, t2100: F, t3450: F, t831: F, t1592: F, t1872: F, t5375: F, t591: F, t4111: F, t5378: F, t5386: F, t5391: F, t1869: F, t8337: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12234 = t802 * t2852;
    let t12239 = t161 * t1554 * t2100;
    let t12240 = t12239 / 45.0;
    let t12245 = t831 * t3450;
    let t12246 = t12245 / 45.0;
    let t12252 = t1872 * t1592;
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12310 = t5386 * t591;
    let t12311 = 4.0 / 3.0 * t12310;
    let t12312 = t5391 * t4111;
    let t12313 = 2e-21 * t12312;
    let t12329 = t8337 * t1869;
    (t12234, t12240, t12246, t12252, t12304, t12306, t12311, t12313, t12329)
}
