//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1122/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1122<F: Float>(t5021: F, t6830: F, t6378: F, t954: F, t10056: F, t2334: F, t951: F, t6383: F, t9410: F, t331: F, t6818: F, t6821: F, t10216: F, t1371: F, t15756: F, t16300: F, t16306: F, t16330: F, t16334: F, t25: F, t3587: F, t589: F) -> (F, F, F, F, F) {
    let t16445 = t5021 * t6830;
    let t16447 = t6378 * t954;
    let t16452 = t10056 * t2334 * t951;
    let t16456 = t6383 * t954;
    let t16461 = t9410 * t2334 * t951;
    let t16468 = t331 * t6818;
    let t16470 = t331 * t6821;
    let t16484 = -0.03851851851851852 * t16445 + 0.013333333333333334 * t25 * t1371 * t16447 + 0.035555555555555556 * t25 * t3587 * t16452 - 0.002962962962962963 * t25 * t3587 * t16456 - 0.006913580246913581 * t25 * t10216 * t16461 - 0.08 * t25 * t1371 * t15756 - 0.017777777777777778 * t16468 + 0.002962962962962963 * t16470 + 0.02666666666666667 * t25 * t589 * t16330 + 0.013333333333333334 * t25 * t589 * t16334 + 0.013333333333333334 * t25 * t1371 * t16300 - 0.0044444444444444444 * t25 * t1371 * t16306;
    (t16447, t16452, t16456, t16461, t16484)
}
