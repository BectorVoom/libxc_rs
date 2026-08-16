//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1387/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1387<F: Float>(t15760: F, t15763: F, t15765: F, t15771: F, t15773: F, t15775: F, t15777: F, t15780: F, t15784: F, t15786: F, t15789: F, t15792: F, t15794: F, t15796: F, t15797: F) -> F {
    let t18181 = -t15760 - t15763 + t15765 + t15771 - t15773 + t15775 + t15777 + t15780 + t15784 + t15786 + t15789 + t15792 + t15794 + t15796 - t15797;
    t18181
}
