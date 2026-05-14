//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 727/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk727<F: Float>(t1296: F, t2238: F, t2722: F, t2730: F, t3632: F, t378: F, t5834: F, t7043: F, t7326: F, t7334: F, t7337: F, t7351: F, t74: F, t787: F, t387: F, t2798: F, t2800: F, t2802: F) -> (F, F, F) {
    let t7353 = 6.0 * t1296 * t7337 - 3.0 * t2238 * t2730 + 6.0 * t5834 * t2722 - 6.0 * t3632 * t7334 - t378 * t7351 - 3.0 * t7043 * t787 + t7326 * t74;
    let t7354 = t7353 * t387;
    let t7364 = t2798 + t2800 + t2802;
    (t7353, t7354, t7364)
}
