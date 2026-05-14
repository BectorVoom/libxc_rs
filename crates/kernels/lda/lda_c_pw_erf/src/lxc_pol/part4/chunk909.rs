//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 909/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk909<F: Float>(t2660: F, t611: F, t225: F, t6039: F, t231: F, t5365: F, t5373: F, t5380: F, t5399: F, t5411: F, t5423: F, t6995: F, t7001: F, t7006: F, t7009: F, t7011: F, t7014: F, t7015: F, t7018: F, t7020: F) -> (F, F) {
    let t7278 = t2660 * t611;
    let t7280 = t6039 * t225;
    let t7283 = -t6995 - t7001 + t7006 + t7009 - t7011 - t7014 - t7015 - t5365 + t5373 - t5380 + t5399 + 4.0 / 3.0 * t7278 + 4.0 / 3.0 * t7280 * t231 + t5411 - t5423 - t7018 + t7020;
    (t7280, t7283)
}
