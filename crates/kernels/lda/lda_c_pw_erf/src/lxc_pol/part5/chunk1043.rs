//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1043/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1043<F: Float>(t544: F, t7661: F, t184: F, t202: F, t7674: F, t551: F, t17684: F, t17687: F, t17690: F, t17692: F, t17694: F, t17697: F, t17699: F, t17709: F, t17715: F, t21717: F, t21719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21721 = 2.0 / 15.0 * t7661 * t544;
    let t21723 = t202 * t7674 * t184;
    let t21725 = 4.0 / 15.0 * t21723 * t551;
    let t21726 = 16.0 / 45.0 * t17684;
    let t21727 = 32.0 / 45.0 * t17687;
    let t21728 = 64.0 / 45.0 * t17690;
    let t21729 = 16.0 / 45.0 * t17692;
    let t21730 = 32.0 / 45.0 * t17694;
    let t21731 = 32.0 / 45.0 * t17697;
    let t21732 = 32.0 / 45.0 * t17699;
    let t21733 = 16.0 / 135.0 * t17709;
    let t21734 = 32.0 / 45.0 * t17715;
    let t21735 = -t21717 - t21719 - t21721 + t21725 - t21726 + t21727 - t21728 - t21729 + t21730 - t21731 + t21732 + t21733 + t21734;
    (t21721, t21725, t21726, t21727, t21728, t21729, t21730, t21731, t21732, t21733, t21734, t21735)
}
