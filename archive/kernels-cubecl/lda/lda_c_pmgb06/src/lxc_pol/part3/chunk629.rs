//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 629/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk629<F: Float>(t3530: F, t69: F, t3533: F, t1227: F, t342: F, t3585: F, t2247: F, t2248: F, t3505: F, t3508: F, t3517: F, t3525: F, t3561: F, t3578: F, t3580: F, t3590: F, t3602: F, t3604: F, t3607: F, t3613: F, t3643: F) -> (F, F, F, F, F) {
    let t3644 = t69 * t3530;
    let t3646 = t69 * t3533;
    let t3650 = t342 * t1227;
    let t3654 = t69 * t3585;
    let t3656 = -F::cast_from(1.724255_f64) * t69 * t3561 - t3643 - F::cast_from(2.2990066666666666_f64) * t3644 + F::cast_from(1.724255_f64) * t3646 - t3505 - t3613 + t3508 - F::cast_from(20.69106_f64) * t69 * t3590 + F::cast_from(15.518295_f64) * t2247 * t2248 * t3650 - F::cast_from(5.172765_f64) * t3654 - t3517 + t3578 + t3525 + t3580 - t3607 - t3602 - t3604;
    (t3644, t3646, t3650, t3654, t3656)
}
