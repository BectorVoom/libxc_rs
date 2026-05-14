//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 949/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk949<F: Float>(t12396: F, t12402: F, t19618: F, t350: F, t7482: F, t7490: F, t19871: F, t36: F, t453: F, t103: F, t13619: F, t13621: F, t15416: F, t15418: F, t15423: F, t15435: F, t15644: F, t15654: F, t15663: F, t15671: F, t15675: F, t15677: F, t1619: F, t19490: F, t19782: F, t19802: F, t2060: F, t473: F, t9724: F) -> (F, F, F, F, F) {
    let t19918 = t12396 * t12402 * t19618;
    let t19920 = t350 * t7482;
    let t19922 = t350 * t7490;
    let t19925 = t36 * t453 * t19871;
    let t19929 = 0.08 * t2060 * t473 * t19782 + 0.013333333333333334 * t103 * t473 * t19802 - 0.0022222222222222222 * t103 * t1619 * t19490 + 0.044444444444444446 * t13619 - 0.007407407407407408 * t13621 + t9724 - 0.047988888888888886 * t15416 + 0.044444444444444446 * t15644 - 0.03199259259259259 * t15418 + 0.09597777777777777 * t15423 + 0.013333333333333334 * t15654 + 0.035991666666666665 * t15435 - 0.007407407407407408 * t15663 - 0.022222222222222223 * t15671 - 0.64785 * t19918 - 0.047988888888888886 * t19920 + 0.07198333333333333 * t19922 - 0.035991666666666665 * t19925 + 0.005925925925925926 * t15675 + 0.017777777777777778 * t15677;
    (t19918, t19920, t19922, t19925, t19929)
}
