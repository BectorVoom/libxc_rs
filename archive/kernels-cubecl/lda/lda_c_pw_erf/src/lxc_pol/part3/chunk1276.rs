//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1276/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1276<F: Float>(t12617: F, t12622: F, t12624: F, t12626: F, t12628: F, t12630: F, t12632: F, t12634: F, t12636: F, t12638: F, t12640: F, t12643: F, t12645: F) -> F {
    let t15025 = t12617 - t12622 - t12624 - t12626 - t12628 + t12630 + t12632 - t12634 + t12636 - t12638 - t12640 - t12643 + t12645;
    t15025
}
