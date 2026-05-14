//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1319/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1319<F: Float>(t17640: F, t17642: F, t17644: F, t17648: F, t17653: F, t17656: F, t17658: F, t17663: F, t17665: F, t17669: F, t17672: F, t17677: F, t17680: F, t17681: F, t17685: F, t17688: F, t17691: F) -> (F,) {
    let t19265 = -t17640 - t17642 + t17644 + t17648 - t17653 - t17656 - t17658 - t17663 - t17665 + t17669 - t17672 - t17677 + t17680 - t17681 - t17685 - t17688 + t17691;
    (t19265,)
}
