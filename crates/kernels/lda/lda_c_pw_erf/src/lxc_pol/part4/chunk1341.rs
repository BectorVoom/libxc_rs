//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1341/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1341<F: Float>(t18656: F, t18658: F, t18659: F, t18660: F, t18661: F, t18662: F, t18663: F, t18664: F, t18665: F, t18666: F, t18668: F, t18672: F, t18674: F, t18676: F, t18678: F, t18683: F, t18694: F) -> (F,) {
    let t19331 = -t18656 - t18658 - t18659 - t18660 - t18661 - t18662 + t18663 + t18664 - t18665 - t18666 + t18668 + t18672 - t18674 + t18676 + t18678 + t18683 + t18694;
    (t19331,)
}
