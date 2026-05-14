//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1023/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1023<F: Float>(t16602: F, t2001: F, t3974: F, t17102: F, t17105: F, t11093: F, t11097: F, t11098: F, t11101: F, t11104: F, t19221: F, t19225: F, t19228: F, t19230: F, t21378: F, t17107: F) -> (F, F, F, F, F) {
    let t21381 = 8.0 / 15.0 * t3974 * t16602 * t2001;
    let t21384 = 8.0 / 45.0 * t17102;
    let t21385 = 16.0 / 15.0 * t17105;
    let t21386 = t19221 + 0.18233333333333332 * t19225 + t19228 + 0.36466666666666664 * t19230 - t21378 - t21381 - 8.0 / 405.0 * t11093 + t11097 + 8.0 / 81.0 * t11098 + t11101 - t11104 - t21384 - t21385;
    let t21387 = 8.0 / 15.0 * t17107;
    (t21381, t21384, t21385, t21386, t21387)
}
