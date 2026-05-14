//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 805/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk805<F: Float>(t446: F, t6127: F, t2553: F, t495: F, t499: F, t493: F, t224: F, t2562: F) -> (F, F, F, F, F) {
    let t6129 = t6127 * t446 / 45.0;
    let t6130 = t495 * t2553;
    let t6131 = t6130 * t499;
    let t6133 = t493 * t6131 / 45.0;
    let t6134 = t2562 * t224;
    (t6129, t6130, t6131, t6133, t6134)
}
