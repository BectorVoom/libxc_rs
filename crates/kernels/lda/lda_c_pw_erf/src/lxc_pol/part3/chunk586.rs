//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 586/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk586<F: Float>(t1349: F, t3605: F, t11: F, t3610: F, t3625: F, t3627: F, t3629: F, t3631: F, t3635: F, t3639: F, t3641: F, t3644: F, t3646: F, t3623: F, t582: F, t186: F) -> (F, F, F, F, F, F, F) {
    let t3648 = t1349 * t3605;
    let t3649 = t11 * t3648;
    let t3651 = t1349 * t3610;
    let t3652 = t11 * t3651;
    let t3654 = -0.035991666666666665 * t3625 - 0.047988888888888886 * t3627 + 0.035991666666666665 * t3629 + 0.023994444444444443 * t3631 - 0.03999074074074074 * t3635 - t3639 - 0.21595 * t3641 + 0.21595 * t3644 - 0.07198333333333333 * t3646 + 0.14396666666666666 * t3649 - 0.07198333333333333 * t3652;
    let t3655 = t3623 + t3654;
    let t3656 = t582 * t3655;
    let t3657 = t186 * t3656;
    (t3648, t3649, t3651, t3652, t3655, t3656, t3657)
}
