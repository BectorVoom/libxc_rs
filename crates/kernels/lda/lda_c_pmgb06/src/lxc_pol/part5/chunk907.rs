//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 907/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk907<F: Float>(t1420: F, t15180: F, t15182: F, t15184: F, t15189: F, t19178: F, t2010: F, t2948: F, t439: F, t5482: F, t6146: F, t6185: F, t6189: F, t6375: F, t6494: F, t6498: F, t7554: F, t7555: F) -> (F,) {
    let t19181 = 2.0 / 15.0 * t439 * t5482 * t6375 + 2.0 / 5.0 * t439 * t6494 * t6185 - 2.0 / 3.0 * t439 * t6498 * t6146 - 8.0 / 15.0 * t2010 * t6494 * t6189 - t1420 * t7555 / 15.0 - t439 * t2948 * t7554 / 15.0 - 4.0 / 45.0 * t15180 - 8.0 / 45.0 * t15182 - 4.0 / 45.0 * t15184 + 4.0 / 45.0 * t19178 - 2.0 / 45.0 * t15189;
    (t19181,)
}
