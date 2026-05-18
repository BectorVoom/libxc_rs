//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1307/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1307<F: Float>(t1476: F, t16354: F, t36: F, t350: F, t6813: F, t405: F, t6882: F, t1080: F, t2389: F, t2918: F, t15200: F, t506: F) -> (F, F, F, F, F, F) {
    let t17175 = t36 * t1476 * t16354;
    let t17177 = t350 * t6813;
    let t17185 = t405 * t6882;
    let t17188 = t2918 * t2389 * t1080;
    let t17190 = t36 * t1476 * t17188;
    let t17193 = t36 * t506 * t15200;
    (t17175, t17177, t17185, t17188, t17190, t17193)
}
