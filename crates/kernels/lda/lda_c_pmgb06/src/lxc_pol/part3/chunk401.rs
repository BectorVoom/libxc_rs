//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 401/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk401<F: Float>(t1467: F, t493: F, t139: F, t409: F, t138: F, t163: F, t350: F, t508: F, t495: F) -> (F, F, F, F, F, F) {
    let t1469 = t493 * t1467 / F::new(27.0);
    let t1470 = t409 * t139;
    let t1472 = t138 * t1470 * t163;
    let t1473 = F::cast_from(0.002518888888888889_f64) * t1472;
    let t1474 = t350 * t508;
    let t1476 = t139 * t495;
    (t1469, t1470, t1472, t1473, t1474, t1476)
}
