//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 958/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk958<F: Float>(t12681: F, t12683: F, t12685: F, t12689: F, t12691: F, t12694: F, t12698: F, t12702: F, t12706: F, t12709: F, t12711: F, t12713: F, t12714: F, t1513: F, t2134: F, t9627: F) -> (F, F, F) {
    let t12716 = t12681 + t12683 - t12685 - t12689 - t12691 - t12694 - t12698 + t12702 + t12706 - t12709 + t12711 - t12713 + 0.0011033703703703704 * t12714;
    let t12717 = t1513 * t2134;
    let t12718 = 8.0 / 15.0 * t12717;
    let t12719 = 8.0 / 15.0 * t9627;
    (t12716, t12718, t12719)
}
