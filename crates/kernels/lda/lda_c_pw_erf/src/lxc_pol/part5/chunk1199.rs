//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1199/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1199<F: Float>(t17692: F, t17694: F, t17697: F, t17699: F, t17709: F, t17715: F, t21717: F, t21719: F, t21721: F, t21725: F, t21726: F, t21727: F, t21728: F) -> (F, F, F, F, F, F, F) {
    let t21729 = F::new(16.0) / F::new(45.0) * t17692;
    let t21730 = F::new(32.0) / F::new(45.0) * t17694;
    let t21731 = F::new(32.0) / F::new(45.0) * t17697;
    let t21732 = F::new(32.0) / F::new(45.0) * t17699;
    let t21733 = F::new(16.0) / F::new(135.0) * t17709;
    let t21734 = F::new(32.0) / F::new(45.0) * t17715;
    let t21735 = -t21717 - t21719 - t21721 + t21725 - t21726 + t21727 - t21728 - t21729 + t21730 - t21731 + t21732 + t21733 + t21734;
    (t21729, t21730, t21731, t21732, t21733, t21734, t21735)
}
