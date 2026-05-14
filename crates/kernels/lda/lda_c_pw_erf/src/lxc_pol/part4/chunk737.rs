//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 737/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk737<F: Float>(t1960: F, t595: F, t1982: F, t544: F, t1498: F, t835: F, t2100: F, t511: F, t558: F, t588: F, t1371: F, t4671: F, t4684: F, t589: F, t190: F, t25: F, t3469: F, t3579: F, t3581: F, t3583: F, t3600: F, t3601: F, t3627: F, t3629: F, t3631: F, t3639: F, t3646: F, t4673: F, t4691: F, t4695: F, t4699: F) -> (F, F, F, F, F, F, F, F) {
    let t4966 = 4.0 / 15.0 * t1960 * t595;
    let t4968 = 4.0 / 15.0 * t1982 * t544;
    let t4970 = 2.0 / 15.0 * t1498 * t835;
    let t4972 = 4.0 / 15.0 * t511 * t2100;
    let t4981 = t588 * t558;
    let t4988 = t1371 * t4671;
    let t4991 = t589 * t4684;
    let t4995 = -t3600 - t3639 - 0.014814814814814815 * t3579 + 0.0044444444444444444 * t3581 + 0.0014814814814814814 * t3583 - 0.008888888888888889 * t3601 - 0.03199259259259259 * t3627 + 0.011997222222222222 * t3629 + 0.007998148148148148 * t3631 - 0.023994444444444443 * t3646 + 0.013333333333333334 * t190 * t3469 * t4981 + 0.07198333333333333 * t4699 + 0.07198333333333333 * t4695 - 0.2879333333333333 * t4691 + 0.013333333333333334 * t25 * t4988 - 0.04 * t25 * t4991 + 0.14396666666666666 * t4673;
    (t4966, t4968, t4970, t4972, t4981, t4988, t4991, t4995)
}
