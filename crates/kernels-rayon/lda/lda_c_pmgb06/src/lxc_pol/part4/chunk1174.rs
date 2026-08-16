//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1174/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1174(t1069: f64, t6150: f64, t1525: f64, t36: f64, t1531: f64, t332: f64, t5961: f64, t453: f64, t12393: f64, t15413: f64, t15416: f64, t15418: f64, t15421: f64, t15423: f64, t15429: f64, t15433: f64, t15435: f64, t15438: f64, t9178: f64) -> (f64, f64, f64, f64, f64) {
    let t15440 = t6150 * t1069;
    let t15442 = t36 * t1525 * t15440;
    let t15445 = t1531 * t5961 * t332;
    let t15447 = t36 * t453 * t15445;
    let t15449 = 0.002099074074074074_f64 * t15413 + 0.005037777777777778_f64 * t12393 - t9178 + 0.0008396296296296296_f64 * t15416 + 0.000559753086419753_f64 * t15418 + 0.005037777777777778_f64 * t15421 - 0.0016792592592592592_f64 * t15423 + 0.010075555555555556_f64 * t15429 - 0.030226666666666666_f64 * t15433 - 0.0012594444444444445_f64 * t15435 - 0.015113333333333333_f64 * t15438 + 0.04534_f64 * t15442 - 0.007556666666666666_f64 * t15447;
    (t15440, t15442, t15445, t15447, t15449)
}
