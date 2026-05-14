//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 769/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk769<F: Float>(t3516: F, t7635: F, t25: F, t3472: F, t3543: F, t4600: F, t7641: F, t7645: F, t7758: F, t7761: F, t7764: F, t7767: F, t7770: F, t5076: F, t6545: F, t6547: F, t6549: F, t6551: F, t6553: F, t6555: F, t7637: F, t7649: F, t7653: F, t7657: F) -> (F, F, F) {
    let t7773 = t3516 * t7635;
    let t7779 = 0.013333333333333334 * t25 * t7758 - 0.006666666666666667 * t25 * t7761 - 0.04 * t25 * t7764 + 0.04 * t25 * t7767 - 0.006666666666666667 * t25 * t7770 - 0.002962962962962963 * t25 * t7773 - t3472 - 0.047988888888888886 * t4600 - t3543 + 0.14396666666666666 * t7641 - 0.07198333333333333 * t7645;
    let t7791 = -0.21595 * t7649 + 0.21595 * t7653 - 0.07198333333333333 * t6545 + 0.035991666666666665 * t6547 + 0.023994444444444443 * t6549 + 0.0044444444444444444 * t6551 - 0.02666666666666667 * t6553 + 0.013333333333333334 * t6555 - 0.035991666666666665 * t7657 - 0.03999074074074074 * t7637 - 0.022222222222222223 * t5076;
    (t7773, t7779, t7791)
}
