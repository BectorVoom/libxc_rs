//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 584/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk584<F: Float>(t2549: F, t506: F, t36: F, t1473: F, t1818: F, t2543: F, t2547: F) -> (F, F, F) {
    let t2550 = t506 * t2549;
    let t2551 = t36 * t2550;
    let t2553 = -t1473 - F::cast_from(0.0012594444444444445_f64) * t1818 + F::cast_from(0.0012594444444444445_f64) * t2543 - F::cast_from(0.003778333333333333_f64) * t2547 + F::cast_from(0.0018891666666666666_f64) * t2551;
    (t2550, t2551, t2553)
}
