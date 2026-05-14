//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 908/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk908<F: Float>(t1: F, t10152: F, t10288: F, t1444: F, t15196: F, t15208: F, t15216: F, t15237: F, t15244: F, t15248: F, t1972: F, t2010: F, t2864: F, t439: F, t493: F, t6399: F, t6403: F, t6522: F, t7558: F, t7559: F, t7562: F) -> (F,) {
    let t19204 = -4.0 / 45.0 * t15196 + 4.0 / 45.0 * t15208 + 4.0 / 45.0 * t15216 - 2.0 / 45.0 * t15237 - 4.0 / 45.0 * t15244 - 4.0 / 45.0 * t15248 + 2.0 / 15.0 * t439 * t10288 * t7562 + 4.0 / 15.0 * t2010 * t2864 * t6522 * t1 + 2.0 / 15.0 * t1972 * t6399 + 2.0 / 5.0 * t1972 * t6403 + 2.0 / 15.0 * t1444 * t7559 + 2.0 / 15.0 * t493 * t10152 * t7558;
    (t19204,)
}
