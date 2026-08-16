//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1074/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1074(t12396: f64, t12402: f64, t19618: f64, t350: f64, t7482: f64, t7490: f64, t19871: f64, t36: f64, t453: f64, t103: f64, t13619: f64, t13621: f64, t15416: f64, t15418: f64, t15423: f64, t15435: f64, t15644: f64, t15654: f64, t15663: f64, t15671: f64, t15675: f64, t15677: f64, t1619: f64, t19490: f64, t19782: f64, t19802: f64, t2060: f64, t473: f64, t9724: f64) -> (f64, f64, f64, f64, f64) {
    let t19918 = t12396 * t12402 * t19618;
    let t19920 = t350 * t7482;
    let t19922 = t350 * t7490;
    let t19925 = t36 * t453 * t19871;
    let t19929 = 0.08_f64 * t2060 * t473 * t19782 + 0.013333333333333334_f64 * t103 * t473 * t19802 - 0.0022222222222222222_f64 * t103 * t1619 * t19490 + 0.044444444444444446_f64 * t13619 - 0.007407407407407408_f64 * t13621 + t9724 - 0.047988888888888886_f64 * t15416 + 0.044444444444444446_f64 * t15644 - 0.03199259259259259_f64 * t15418 + 0.09597777777777777_f64 * t15423 + 0.013333333333333334_f64 * t15654 + 0.035991666666666665_f64 * t15435 - 0.007407407407407408_f64 * t15663 - 0.022222222222222223_f64 * t15671 - 0.64785_f64 * t19918 - 0.047988888888888886_f64 * t19920 + 0.07198333333333333_f64 * t19922 - 0.035991666666666665_f64 * t19925 + 0.005925925925925926_f64 * t15675 + 0.017777777777777778_f64 * t15677;
    (t19918, t19920, t19922, t19925, t19929)
}
