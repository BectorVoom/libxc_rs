//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1048/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1048<F: Float>(t4481: F, t643: F, t4516: F, t638: F, t1101: F, t2160: F, t2158: F, t2799: F, t898: F, t2801: F, t3947: F, t3952: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11065 = t643 * t4481;
    let t11067 = t638 * t4516;
    let t11070 = t643 * t4516;
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11099 = t3947 * t898;
    let t11101 = t3952 * t898;
    (t11065, t11067, t11070, t11090, t11092, t11095, t11097, t11099, t11101)
}
