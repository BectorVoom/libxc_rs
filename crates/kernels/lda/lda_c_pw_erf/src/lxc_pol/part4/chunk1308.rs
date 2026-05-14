//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1308/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1308<F: Float>(t11093: F, t11097: F, t11098: F, t11101: F, t11104: F, t11105: F, t17100: F, t17103: F, t17104: F, t17106: F, t17108: F, t17110: F, t17113: F, t17115: F, t17118: F, t17121: F, t17124: F) -> (F,) {
    let t19238 = -16.0 / 405.0 * t11093 + t11097 - t17100 + 16.0 / 81.0 * t11098 + t11101 - t11104 + 2.0 / 135.0 * t11105 - t17103 - t17104 + t17106 + t17108 + t17110 + t17113 + t17115 + t17118 + t17121 - t17124;
    (t19238,)
}
