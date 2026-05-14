//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 934/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk934<F: Float>(t13026: F, t13027: F, t19618: F, t2377: F, t822: F, t477: F, t12519: F, t5083: F, t332: F, t13031: F, t1: F, t6637: F, t13020: F, t5084: F, t497: F, t7857: F) -> (F, F, F, F, F, F, F, F) {
    let t19621 = 8.0 / 27.0 * t13026 * t13027 * t19618;
    let t19622 = t2377 * t822;
    let t19623 = t19622 * t477;
    let t19626 = 2.0 / 9.0 * t5083 * t12519 * t19623;
    let t19627 = t19622 * t332;
    let t19630 = 8.0 / 27.0 * t13026 * t13031 * t19627;
    let t19631 = t6637 * t1;
    let t19634 = 4.0 / 9.0 * t13020 * t5084 * t19631;
    let t19635 = t7857 * t497;
    (t19621, t19623, t19626, t19627, t19630, t19631, t19634, t19635)
}
