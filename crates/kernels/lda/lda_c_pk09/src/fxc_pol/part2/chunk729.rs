//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 729/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk729<F: Float>(t3317: F, t3335: F, t3342: F, t3918: F, t3920: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t3319: F, t3323: F, t3326: F, t3917: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t8155 = -0.6280807972466558 * t7801 - 0.9421211958699838 * t7805 - 0.9421211958699838 * t7809 - 0.9421211958699838 * t7811 - 0.9421211958699838 * t7814 - 0.9421211958699838 * t7817 - 0.9421211958699838 * t7834 - 0.9421211958699838 * t3335 - 0.6280807972466558 * t3342 + t3918 - t3920 + 0.9421211958699838 * t3317;
    let t8168 = 0.9421211958699838 * t3319 + 0.6280807972466558 * t3323 + 0.6280807972466558 * t3326 + t3917 + 1.8842423917399675 * t7896 + 0.9421211958699838 * t7919 + 0.9421211958699838 * t7923 + 0.9421211958699838 * t7926 + 0.9421211958699838 * t7928 + 0.9421211958699838 * t7931 + 0.9421211958699838 * t7935 + 0.6280807972466558 * t7939 + 0.6280807972466558 * t7942;
    (t8155, t8168)
}
