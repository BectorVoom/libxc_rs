//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 365/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk365<F: Float>(t1152: F, t1193: F, t1354: F, t117: F, t123: F, t191: F, t740: F, t315: F, t550: F, t109: F, t186: F, t55: F) -> (F, F, F, F) {
    let t1356 = 0.0004954275694490498 * t1152 * t1193 * t1354;
    let t1360 = 0.02394846802050922 * t123 * t740 * t191 * t117;
    let t1363 = t123 * t315 * t550 * t117;
    let t1366 = t55 * t109 * t186;
    (t1356, t1360, t1363, t1366)
}
