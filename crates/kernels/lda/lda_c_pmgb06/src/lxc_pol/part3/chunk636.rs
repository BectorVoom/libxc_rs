//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 636/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk636<F: Float>(t4065: F, t4071: F, t4122: F, t4152: F, t4155: F, t4156: F, t4158: F, t4166: F, t122: F, t1669: F, t610: F, t1735: F, t569: F, t107: F, t2786: F, t290: F) -> (F, F, F, F) {
    let t4169 = t4065 + t4071 + t4122 + t4152 + t4155 + t4156 + t4158 + t4166;
    let t4174 = t122 * t1669 * t610;
    let t4177 = t122 * t569 * t1735;
    let t4181 = 4.429070076315393 * t107 * t2786 * t290;
    (t4169, t4174, t4177, t4181)
}
