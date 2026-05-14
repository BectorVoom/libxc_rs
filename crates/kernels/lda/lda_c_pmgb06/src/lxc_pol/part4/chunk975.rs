//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 975/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk975<F: Float>(t4103: F, t872: F, t132: F, t435: F, t4978: F, t5040: F, t4974: F, t432: F, t5326: F, t486: F, t5044: F, t1554: F, t161: F, t1836: F, t1912: F, t3223: F) -> (F, F, F, F, F, F, F, F) {
    let t12804 = t872 * t4103;
    let t12807 = t132 * t435 * t4978;
    let t12816 = t132 * t435 * t5040;
    let t12822 = t132 * t435 * t4974;
    let t12825 = t432 * t5326;
    let t12828 = t486 * t5044;
    let t12831 = t161 * t1554 * t1836;
    let t12868 = t3223 * t1912;
    (t12804, t12807, t12816, t12822, t12825, t12828, t12831, t12868)
}
