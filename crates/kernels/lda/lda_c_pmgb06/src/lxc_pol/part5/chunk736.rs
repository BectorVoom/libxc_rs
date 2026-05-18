//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 736/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk736<F: Float>(t2617: F, t405: F, t2620: F, t2614: F, t525: F, t6827: F, t1576: F, t6503: F, t3358: F, t6508: F, t6512: F, t6402: F) -> (F, F, F, F, F, F, F, F) {
    let t6873 = t405 * t2617;
    let t6875 = t405 * t2620;
    let t6877 = t405 * t2614;
    let t6879 = t525 * t6827;
    let t6882 = t1576 * t6503;
    let t6885 = t3358 * t6508;
    let t6888 = t1576 * t6512;
    let t6891 = t525 * t6402;
    (t6873, t6875, t6877, t6879, t6882, t6885, t6888, t6891)
}
