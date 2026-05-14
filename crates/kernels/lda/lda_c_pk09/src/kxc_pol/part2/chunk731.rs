//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 731/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk731<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3946: F, t3949: F, t3950: F, t3951: F, t7851: F, t7855: F, t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t3960: F, t3967: F, t3969: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> (F, F) {
    let t8202 = 6.0 * t7851 + 6.0 * t7855 - 0.505765839233979 * t3335 - 0.337177226155986 * t3342 + 12.0 * t3384 + 12.0 * t3388 - 12.0 * t3393 + t3946 + t3949 + t3950 - t3951 + 0.505765839233979 * t3317 + 0.505765839233979 * t3319;
    let t8214 = 0.337177226155986 * t3323 + 0.337177226155986 * t3326 + t3960 + 12.0 * t7870 - 12.0 * t7875 + 12.0 * t7879 - 12.0 * t7884 + 12.0 * t7888 + 8.0 * t3424 + 8.0 * t3426 - 8.0 * t3428 + t3967 + t3969;
    (t8202, t8214)
}
