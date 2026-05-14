//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 980/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk980<F: Float>(t20391: F, t5138: F, t5139: F, t12529: F, t13300: F, t19314: F, t5077: F, t5078: F, t6364: F, t1864: F, t2605: F, t1859: F, t5083: F, t20380: F, t20382: F, t20386: F, t20390: F, t20394: F, t20397: F, t20400: F) -> (F, F, F, F, F, F) {
    let t20403 = t5138 * t5139 * t20391 / 9.0;
    let t20406 = 8.0 / 27.0 * t12529 * t13300 * t19314;
    let t20409 = 4.0 / 15.0 * t5077 * t5078 * t6364;
    let t20412 = 4.0 / 15.0 * t5077 * t2605 * t1864;
    let t20415 = 2.0 / 9.0 * t5083 * t2605 * t1859;
    let t20416 = t20380 + t20382 + t20386 + t20390 + t20394 + t20397 - t20400 - t20403 + t20406 + t20409 - t20412 + t20415;
    (t20403, t20406, t20409, t20412, t20415, t20416)
}
