//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 561/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk561<F: Float>(t144: F, t3259: F, t153: F, t3092: F, t1435: F, t458: F, t1592: F, t1555: F, t486: F, t186: F, t409: F, t55: F) -> (F, F, F, F, F, F) {
    let t3260 = t3259 * t144;
    let t3261 = t153 * t3092;
    let t3279 = t1435 * t458;
    let t3290 = t458 * t1592;
    let t3306 = t486 * t1555;
    let t3309 = t55 * t409 * t186;
    (t3260, t3261, t3279, t3290, t3306, t3309)
}
