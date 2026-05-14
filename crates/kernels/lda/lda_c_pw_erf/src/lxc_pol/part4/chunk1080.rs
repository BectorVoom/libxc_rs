//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1080/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1080<F: Float>(t11914: F, t13771: F, t15728: F, t9244: F, t9246: F, t15687: F, t15689: F, t15693: F, t15696: F, t15698: F, t15702: F, t15707: F, t15711: F, t15716: F, t15720: F, t15724: F, t15726: F, t9250: F) -> (F, F, F, F) {
    let t15731 = 256.0 / 81.0 * t13771 * t11914 * t15728;
    let t15732 = 8.0 / 135.0 * t9244;
    let t15733 = 32.0 / 405.0 * t9246;
    let t15734 = -t15687 + t15689 - t15693 - t15696 + t15698 + t15702 - t15707 + t15711 - t15716 + t15720 - t15724 - t15726 - t15731 - t15732 + t15733 - t9250;
    (t15731, t15732, t15733, t15734)
}
