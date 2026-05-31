//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 494/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk494<F: Float>(t2489: F, t493: F, t1558: F, t2377: F, t442: F) -> (F, F, F) {
    let t2491 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t2489;
    let t2492 = t1558 * t2377;
    let t2493 = t442 * t2492;
    (t2491, t2492, t2493)
}
