//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1316/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1316<F: Float>(t17559: F, t17560: F, t17561: F, t17563: F, t17564: F, t17565: F, t17566: F, t17567: F, t17568: F, t17569: F, t17570: F, t17571: F, t17572: F, t17573: F, t17574: F, t17575: F, t17576: F) -> (F,) {
    let t19261 = t17559 - t17560 - t17561 + t17563 + t17564 - t17565 - t17566 + t17567 + t17568 + t17569 + t17570 - t17571 + t17572 + t17573 + t17574 + t17575 - t17576;
    (t19261,)
}
