//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1281/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1281<F: Float>(t15549: F, t15551: F, t15556: F, t15558: F, t15560: F, t15562: F, t15565: F, t15567: F, t15569: F, t15571: F, t15574: F, t15576: F, t15578: F, t15581: F, t15584: F, t15588: F, t15589: F) -> (F,) {
    let t19106 = -t15549 - t15551 + t15556 - t15558 - t15560 + t15562 + t15565 + t15567 - t15569 - t15571 + t15574 + t15576 + t15578 - t15581 - t15584 + t15588 - t15589;
    (t19106,)
}
