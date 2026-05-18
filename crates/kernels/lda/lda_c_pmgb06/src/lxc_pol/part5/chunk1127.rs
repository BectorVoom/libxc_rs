//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1127/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1127<F: Float>(t13244: F, t20515: F, t20516: F, t20517: F, t20518: F, t20519: F, t20520: F, t20521: F, t20523: F, t20525: F, t20529: F, t16563: F, t1893: F, t5077: F) -> (F, F) {
    let t20530 = t20515 + t20516 + t20517 + t20518 + t20519 - t20520 + t20521 - t20523 - t20525 - t20529 - t13244;
    let t20533 = F::new(2.0) / F::new(15.0) * t5077 * t16563 * t1893;
    (t20530, t20533)
}
