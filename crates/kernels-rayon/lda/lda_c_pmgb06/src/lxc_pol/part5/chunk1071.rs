//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1071/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1071(t405: f64, t7779: f64, t7785: f64, t103: f64, t12329: f64, t12337: f64, t12366: f64, t13558: f64, t13561: f64, t13565: f64, t13566: f64, t13570: f64, t13574: f64, t13595: f64, t15589: f64, t1619: f64, t19471: f64, t19475: f64, t19618: f64, t19754: f64, t19758: f64, t19762: f64, t19766: f64, t19791: f64, t2060: f64, t3404: f64, t473: f64, t9693: f64, t9702: f64) -> f64 {
    let t19852 = t405 * t7779;
    let t19854 = t405 * t7785;
    let t19862 = 0.019753086419753086_f64 * t9702 + 0.035555555555555556_f64 * t103 * t3404 * t19791 + 0.08_f64 * t2060 * t1619 * t19754 - 0.006666666666666667_f64 * t103 * t1619 * t19758 - 0.013333333333333334_f64 * t2060 * t1619 * t19762 - 0.006913580246913581_f64 * t103 * t9693 * t19471 - 0.017777777777777778_f64 * t2060 * t3404 * t19475 + 0.16_f64 * t103 * t473 * t19766 - 0.12_f64 * t13565 * t13570 * t19618 + 0.04_f64 * t13565 * t13574 * t19618 - 0.008888888888888889_f64 * t13565 * t13566 * t19618 - 0.02666666666666667_f64 * t19852 + 0.0044444444444444444_f64 * t19854 + 0.05925925925925926_f64 * t13558 - 0.044444444444444446_f64 * t13561 - 0.09597777777777777_f64 * t12329 + 0.11197407407407407_f64 * t12337 + 0.09597777777777777_f64 * t12366 - t13595 + 0.08_f64 * t15589;
    t19862
}
