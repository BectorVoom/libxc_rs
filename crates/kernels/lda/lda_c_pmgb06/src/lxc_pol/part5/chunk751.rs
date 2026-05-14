//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 751/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk751<F: Float>(t439: F, t7696: F, t4837: F, t4845: F, t5045: F, t5047: F, t183: F, t7364: F, t5049: F, t5052: F, t188: F, t3007: F, t5633: F, t7683: F, t7687: F, t7689: F, t7692: F, t7694: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7698 = t439 * t7696 / 5.0;
    let t7700 = t4837 / 45.0;
    let t7701 = t4845 / 45.0;
    let t7702 = t5045 / 45.0;
    let t7703 = t5047 / 45.0;
    let t7704 = t7364 * t183;
    let t7707 = t5049 / 45.0;
    let t7708 = t5052 / 45.0;
    let t7709 = t7683 + t7687 + t7689 + t7692 + t7694 + t7698 + 4.0 * t5633 + t7700 + t7701 - t7702 - t7703 + 4.0 / 3.0 * t7704 * t188 - t7707 - t7708 + t3007;
    (t7698, t7700, t7701, t7702, t7703, t7704, t7707, t7708, t7709)
}
