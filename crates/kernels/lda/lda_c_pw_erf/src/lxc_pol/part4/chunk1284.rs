//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1284/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1284<F: Float>(t15693: F, t15696: F, t15698: F, t15702: F, t15707: F, t15711: F, t15716: F, t15720: F, t15724: F, t15726: F, t15731: F, t15732: F, t15733: F, t15735: F, t15737: F, t15740: F, t9250: F) -> (F,) {
    let t19112 = -t15693 - t15696 + t15698 + t15702 - t15707 + t15711 - t15716 + t15720 - t15724 - t15726 - t15731 - t15732 + t15733 - t9250 - t15735 - t15737 + t15740;
    (t19112,)
}
