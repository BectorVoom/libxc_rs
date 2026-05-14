//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1168/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1168<F: Float>(t11775: F, t11781: F, t11875: F, t15777: F, t15779: F, t15785: F, t15788: F, t15792: F, t15796: F, t15825: F, t16405: F, t9772: F, t9782: F, t9784: F, t2494: F, t933: F) -> (F, F) {
    let t17215 = 0.14396666666666666 * t11775 + 0.05925925925925926 * t11781 + 0.03950617283950617 * t9772 - 0.007407407407407408 * t9782 - 0.0024691358024691358 * t9784 - 0.015996296296296297 * t15777 + 0.03199259259259259 * t15779 + 0.5758666666666666 * t15785 + 0.023994444444444443 * t15788 - 0.19195555555555555 * t15792 - 0.32 * t16405 * t11875 * t15825 - 0.21595 * t15796;
    let t17226 = t933 * t2494;
    (t17215, t17226)
}
