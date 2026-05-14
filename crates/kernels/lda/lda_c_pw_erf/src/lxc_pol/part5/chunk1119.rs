//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1119/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1119<F: Float>(t473: F, t483: F, t485: F, t7337: F, t11678: F, t15501: F, t20670: F, t20674: F, t20676: F, t20678: F, t20679: F, t20680: F, t20681: F, t20682: F, t20683: F, t20684: F, t20685: F) -> (F, F) {
    let t23185 = t473 * t7337 * t483 * t485;
    let t23191 = 8.0 * t15501 + t20670 + t20674 + t20676 + t20678 + t11678 - t20679 + t20680 + t20681 + t20682 - t20683 - t20684 - t20685;
    (t23185, t23191)
}
