//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 949/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk949<F: Float>(t132: F, t435: F, t4974: F, t9644: F, t432: F, t5326: F, t9754: F, t486: F, t5044: F, t1554: F, t161: F, t1836: F, t9760: F, t9762: F, t9765: F, t9771: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12822 = t132 * t435 * t4974;
    let t12823 = 2.0 / 15.0 * t12822;
    let t12824 = 2.0 / 15.0 * t9644;
    let t12825 = t432 * t5326;
    let t12826 = 2.0 / 15.0 * t12825;
    let t12827 = 2.0 / 15.0 * t9754;
    let t12828 = t486 * t5044;
    let t12829 = t12828 / 45.0;
    let t12831 = t161 * t1554 * t1836;
    let t12832 = t12831 / 45.0;
    let t12833 = t9760 / 15.0;
    let t12834 = 4.0 / 135.0 * t9762;
    let t12835 = 4.0 / 135.0 * t9765;
    let t12836 = t9771 / 15.0;
    (t12823, t12824, t12826, t12827, t12829, t12832, t12833, t12834, t12835, t12836)
}
