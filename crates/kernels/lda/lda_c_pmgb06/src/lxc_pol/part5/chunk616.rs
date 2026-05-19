//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 616/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk616<F: Float>(t1860: F, t350: F, t1865: F, t2786: F, t30: F) -> (F, F, F, F) {
    let t4637 = t350 * t1860;
    let t4639 = t350 * t1865;
    let t4640 = F::cast_from(0.002518888888888889_f64) * t4639;
    let t4641 = t30 * t2786;
    (t4637, t4639, t4640, t4641)
}
