//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 764/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk764<F: Float>(t5992: F, t769: F, t783: F, t4232: F, t113: F, t301: F, t7364: F, t23: F, t7277: F, t2854: F, t4718: F, t4740: F, t6327: F, t6358: F, t7445: F, t7447: F, t7448: F, t7449: F, t7450: F, t7451: F, t7452: F, t7453: F, t7454: F, t7455: F, t7456: F) -> (F, F, F, F, F, F) {
    let t7917 = t5992 * t769;
    let t7920 = t783 * t769;
    let t7921 = t4232 * t7920;
    let t7934 = t7364 * t113 * t301;
    let t7937 = t7277 * t23;
    let t7945 = -t7445 + t2854 + 2.0 / 45.0 * t4718 + 0.09973633333333333 * t4740 - t7447 - t7448 + t7449 + t7450 + t7451 + t7452 + t7453 + t7454 + t7455 + t7456 + 2.0 / 3.0 * t6327 - 2.0 / 15.0 * t6358;
    (t7917, t7920, t7921, t7934, t7937, t7945)
}
