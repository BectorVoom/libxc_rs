//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1174/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1174<F: Float>(t1069: F, t6150: F, t1525: F, t36: F, t1531: F, t332: F, t5961: F, t453: F, t12393: F, t15413: F, t15416: F, t15418: F, t15421: F, t15423: F, t15429: F, t15433: F, t15435: F, t15438: F, t9178: F) -> (F, F, F, F, F) {
    let t15440 = t6150 * t1069;
    let t15442 = t36 * t1525 * t15440;
    let t15445 = t1531 * t5961 * t332;
    let t15447 = t36 * t453 * t15445;
    let t15449 = F::cast_from(0.002099074074074074_f64) * t15413 + F::cast_from(0.005037777777777778_f64) * t12393 - t9178 + F::cast_from(0.0008396296296296296_f64) * t15416 + F::cast_from(0.000559753086419753_f64) * t15418 + F::cast_from(0.005037777777777778_f64) * t15421 - F::cast_from(0.0016792592592592592_f64) * t15423 + F::cast_from(0.010075555555555556_f64) * t15429 - F::cast_from(0.030226666666666666_f64) * t15433 - F::cast_from(0.0012594444444444445_f64) * t15435 - F::cast_from(0.015113333333333333_f64) * t15438 + F::new(0.04534) * t15442 - F::cast_from(0.007556666666666666_f64) * t15447;
    (t15440, t15442, t15445, t15447, t15449)
}
