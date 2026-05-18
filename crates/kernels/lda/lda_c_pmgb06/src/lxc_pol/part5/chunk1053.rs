//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1053<F: Float>(t13026: F, t13027: F, t19618: F, t2377: F, t822: F, t477: F, t12519: F, t5083: F, t332: F, t13031: F, t1: F, t6637: F) -> (F, F, F, F, F, F) {
    let t19621 = F::new(8.0) / F::new(27.0) * t13026 * t13027 * t19618;
    let t19622 = t2377 * t822;
    let t19623 = t19622 * t477;
    let t19626 = F::new(2.0) / F::new(9.0) * t5083 * t12519 * t19623;
    let t19627 = t19622 * t332;
    let t19630 = F::new(8.0) / F::new(27.0) * t13026 * t13031 * t19627;
    let t19631 = t6637 * t1;
    (t19621, t19623, t19626, t19627, t19630, t19631)
}
