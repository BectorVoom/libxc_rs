//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 930/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk930<F: Float>(t15840: F, t15842: F, t1423: F, t7585: F, t15850: F, t15887: F, t15891: F, t15893: F, t15895: F, t15897: F, t15899: F, t10681: F, t10684: F, t1447: F, t7567: F, t7634: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19532 = 4.0 / 45.0 * t15840;
    let t19533 = 4.0 / 45.0 * t15842;
    let t19534 = t1423 * t7585;
    let t19535 = 16.0 / 243.0 * t19534;
    let t19536 = t15850 / 15.0;
    let t19538 = 8.0 / 45.0 * t15887;
    let t19539 = 4.0 / 45.0 * t15891;
    let t19540 = 4.0 / 45.0 * t15893;
    let t19541 = 8.0 / 45.0 * t15895;
    let t19542 = 4.0 / 45.0 * t15897;
    let t19543 = 4.0 / 45.0 * t15899;
    let t19544 = t19532 + t19533 + t19535 + t19536 - 0.011181742741110338 * t10681 - t10684 + t19538 - t19539 - t19540 - t19541 - t19542 - t19543;
    let t19549 = t1447 * t7567;
    let t19551 = t1447 * t7634;
    (t19532, t19533, t19535, t19536, t19538, t19539, t19540, t19541, t19542, t19543, t19544, t19549, t19551)
}
