//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 718/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk718<F: Float>(t3317: F, t3335: F, t3342: F, t3871: F, t3873: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t3319: F, t3323: F, t3326: F, t3870: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t8013 = -1.0416666666666667 * t7801 - 1.5625 * t7805 - 1.5625 * t7809 - 1.5625 * t7811 - 1.5625 * t7814 - 1.5625 * t7817 - 1.5625 * t7834 - 1.5625 * t3335 - 1.0416666666666667 * t3342 + t3871 - t3873 + 1.5625 * t3317;
    let t8026 = 1.5625 * t3319 + 1.0416666666666667 * t3323 + 1.0416666666666667 * t3326 + t3870 + 3.125 * t7896 + 1.5625 * t7919 + 1.5625 * t7923 + 1.5625 * t7926 + 1.5625 * t7928 + 1.5625 * t7931 + 1.5625 * t7935 + 1.0416666666666667 * t7939 + 1.0416666666666667 * t7942;
    (t8013, t8026)
}
