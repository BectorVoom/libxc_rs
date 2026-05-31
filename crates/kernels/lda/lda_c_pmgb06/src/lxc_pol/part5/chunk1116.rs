//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1116/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1116<F: Float>(t1864: F, t2605: F, t5077: F, t1859: F, t5083: F, t20380: F, t20382: F, t20386: F, t20390: F, t20394: F, t20397: F, t20400: F, t20403: F, t20406: F, t20409: F) -> (F, F, F) {
    let t20412 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t2605 * t1864;
    let t20415 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5083 * t2605 * t1859;
    let t20416 = t20380 + t20382 + t20386 + t20390 + t20394 + t20397 - t20400 - t20403 + t20406 + t20409 - t20412 + t20415;
    (t20412, t20415, t20416)
}
