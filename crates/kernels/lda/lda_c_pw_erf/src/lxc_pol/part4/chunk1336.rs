//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1336/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1336<F: Float>(t15123: F, t15125: F, t15132: F, t15135: F, t15138: F, t15139: F, t15143: F, t15145: F, t15147: F, t18459: F, t18463: F, t18467: F, t18469: F, t18471: F, t18473: F, t18475: F, t18476: F) -> (F,) {
    let t19316 = t18459 + 2.0 / 3.0 * t15123 + 0.36466666666666664 * t15125 + 2.0 / 3.0 * t15132 + 0.12155555555555556 * t15135 + 4.0 / 3.0 * t15138 + 0.4862222222222222 * t15139 + t18463 - t18467 + t18469 + t18471 - t18473 - t18475 + t18476 + 16.0 / 9.0 * t15143 + 4e-21 * t15145 + 4.0 / 3.0 * t15147;
    (t19316,)
}
