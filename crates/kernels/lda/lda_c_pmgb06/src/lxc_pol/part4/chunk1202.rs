//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1202/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1202<F: Float>(t1462: F, t1465: F, t15845: F, t79: F, t486: F, t6843: F, t130: F, t801: F, t5076: F, t5095: F, t5082: F, t5087: F) -> (F, F, F, F, F) {
    let t15849 = F::new(8.0) / F::new(27.0) * t15845 * t1462 * t1465 * t79;
    let t15850 = t486 * t6843;
    let t15851 = F::new(2.0) / F::new(45.0) * t15850;
    let t15854 = t801 * t130;
    let t15855 = t15854 * t5076;
    let t15857 = F::new(8.0) / F::new(45.0) * t15855 * t5095;
    let t15858 = t15854 * t5082;
    let t15860 = F::new(4.0) / F::new(27.0) * t15858 * t5087;
    (t15849, t15851, t15855, t15857, t15860)
}
