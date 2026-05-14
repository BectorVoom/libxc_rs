//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 946/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk946<F: Float>(t405: F, t7779: F, t7785: F, t103: F, t12329: F, t12337: F, t12366: F, t13558: F, t13561: F, t13565: F, t13566: F, t13570: F, t13574: F, t13595: F, t15589: F, t1619: F, t19471: F, t19475: F, t19618: F, t19754: F, t19758: F, t19762: F, t19766: F, t19791: F, t2060: F, t3404: F, t473: F, t9693: F, t9702: F) -> (F,) {
    let t19852 = t405 * t7779;
    let t19854 = t405 * t7785;
    let t19862 = 0.019753086419753086 * t9702 + 0.035555555555555556 * t103 * t3404 * t19791 + 0.08 * t2060 * t1619 * t19754 - 0.006666666666666667 * t103 * t1619 * t19758 - 0.013333333333333334 * t2060 * t1619 * t19762 - 0.006913580246913581 * t103 * t9693 * t19471 - 0.017777777777777778 * t2060 * t3404 * t19475 + 0.16 * t103 * t473 * t19766 - 0.12 * t13565 * t13570 * t19618 + 0.04 * t13565 * t13574 * t19618 - 0.008888888888888889 * t13565 * t13566 * t19618 - 0.02666666666666667 * t19852 + 0.0044444444444444444 * t19854 + 0.05925925925925926 * t13558 - 0.044444444444444446 * t13561 - 0.09597777777777777 * t12329 + 0.11197407407407407 * t12337 + 0.09597777777777777 * t12366 - t13595 + 0.08 * t15589;
    (t19862,)
}
