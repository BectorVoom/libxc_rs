//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1283/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1283<F: Float>(t15644: F, t15646: F, t15648: F, t15650: F, t15654: F, t15658: F, t15660: F, t15662: F, t15666: F, t15671: F, t15673: F, t15675: F, t15677: F, t15679: F, t15681: F, t15687: F, t15689: F) -> (F,) {
    let t19109 = t15644 + t15646 - t15648 + t15650 + t15654 + t15658 + t15660 + t15662 + t15666 + t15671 + t15673 - t15675 - t15677 + t15679 - t15681 - t15687 + t15689;
    (t19109,)
}
