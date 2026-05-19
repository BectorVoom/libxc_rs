//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1032/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1032<F: Float>(t301: F, t3993: F, t413: F, t2789: F, t718: F, t1135: F, t1183: F, t3982: F, t1139: F, t1100: F, t83: F, t113: F) -> (F, F, F, F, F, F, F) {
    let t10609 = t3993 * t413 * t301;
    let t10614 = F::cast_from(0.0011622696607154768_f64) * t718 * t2789 * t301;
    let t10617 = F::cast_from(0.008135887625008338_f64) * t1135 * t1183 * t301;
    let t10623 = t3982 * t413 * t301;
    let t10635 = t1139 * t1183 * t301;
    let t10637 = t1100 * t83;
    let t10640 = F::cast_from(0.03831185177913979_f64) * t10637 * t113 * t301;
    (t10609, t10614, t10617, t10623, t10635, t10637, t10640)
}
