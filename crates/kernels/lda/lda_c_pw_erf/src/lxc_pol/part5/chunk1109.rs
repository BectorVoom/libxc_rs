//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1109/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1109<F: Float>(t15542: F, t15557: F, t15559: F, t15568: F, t15570: F, t11678: F, t20670: F, t20674: F, t20676: F, t20678: F, t20679: F, t20680: F, t20681: F) -> (F, F, F, F, F, F) {
    let t20682 = F::new(8.0) / F::new(45.0) * t15542;
    let t20683 = F::new(16.0) / F::new(15.0) * t15557;
    let t20684 = F::new(16.0) / F::new(15.0) * t15559;
    let t20685 = F::new(16.0) / F::new(135.0) * t15568;
    let t20686 = F::new(16.0) / F::new(135.0) * t15570;
    let t20687 = t20670 + t20674 + t20676 + t20678 + t11678 - t20679 + t20680 + t20681 + t20682 - t20683 - t20684 - t20685 - t20686;
    (t20682, t20683, t20684, t20685, t20686, t20687)
}
