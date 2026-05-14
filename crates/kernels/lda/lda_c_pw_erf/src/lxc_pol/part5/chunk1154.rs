//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1154/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1154<F: Float>(t15311: F, t15312: F, t15315: F, t15316: F, t19986: F, t20046: F, t20056: F, t20071: F, t20080: F, t20088: F, t20096: F, t23363: F, t2991: F, t3005: F, t3019: F, t4305: F, t5695: F, t7: F, t7315: F, t7378: F, t8120: F, t8121: F, t8122: F, t8123: F, t8126: F, t8130: F, t8134: F) -> (F,) {
    let t23368 = -t2991 - t8120 + t8121 - 72.0 * t5695 - t15311 - t15312 + t8122 - t8123 + t3005 - t8126 - t15315 - t15316 + t8130 + t4305 - 2.464579730404 * t7315 + t7378 + t3019 + t8134 + t7 * (t19986 + t20046 + t20056 + t20071 + t20080 + t20088 + t20096 + t23363);
    (t23368,)
}
