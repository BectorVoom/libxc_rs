//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1015/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1015<F: Float>(t331: F, t5004: F, t10053: F, t10066: F, t10079: F, t10090: F, t10216: F, t12153: F, t12264: F, t13290: F, t13368: F, t13598: F, t13655: F, t13659: F, t13661: F, t13663: F, t13665: F, t13667: F, t13675: F, t13677: F, t13679: F, t1371: F, t2061: F, t25: F, t589: F) -> (F,) {
    let t13681 = t331 * t5004;
    let t13692 = -0.047988888888888886 * t10053 + 0.07198333333333333 * t10066 + 0.011997222222222222 * t10079 + 0.11197407407407407 * t10090 + 0.4319 * t13655 - 0.11997222222222222 * t13659 + 0.044444444444444446 * t13661 - 0.007407407407407408 * t13663 + 0.3466666666666667 * t13665 - 0.057777777777777775 * t13667 - 0.08 * t25 * t1371 * t13290 + 0.16 * t25 * t589 * t13368 - 0.02666666666666667 * t13675 - 0.02666666666666667 * t13677 + 0.0044444444444444444 * t13679 + 0.005925925925925926 * t13681 - 0.0022222222222222222 * t25 * t1371 * t12264 + 0.013333333333333334 * t2061 * t1371 * t12153 - 0.006913580246913581 * t25 * t10216 * t13598;
    (t13692,)
}
