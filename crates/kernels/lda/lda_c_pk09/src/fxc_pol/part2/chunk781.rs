//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 781/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk781<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t4187: F, t4190: F, t4191: F, t4192: F, t7851: F, t7855: F, t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t4201: F, t4208: F, t4210: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> (F, F) {
    let t9301 = 4.431130547644593 * t7851 + 4.431130547644593 * t7855 - 0.2946275542389858 * t3335 - 0.1964183694926572 * t3342 + 8.862261095289186 * t3384 + 8.862261095289186 * t3388 - 8.862261095289186 * t3393 + t4187 + t4190 + t4191 - t4192 + 0.2946275542389858 * t3317 + 0.2946275542389858 * t3319;
    let t9313 = 0.1964183694926572 * t3323 + 0.1964183694926572 * t3326 + t4201 + 8.862261095289186 * t7870 - 8.862261095289186 * t7875 + 8.862261095289186 * t7879 - 8.862261095289186 * t7884 + 8.862261095289186 * t7888 + 5.908174063526125 * t3424 + 5.908174063526125 * t3426 - 5.908174063526125 * t3428 + t4208 + t4210;
    (t9301, t9313)
}
