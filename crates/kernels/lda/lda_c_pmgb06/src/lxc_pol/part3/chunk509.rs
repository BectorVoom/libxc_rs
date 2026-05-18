//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 509/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk509<F: Float>(t2148: F, t698: F, t27: F, t897: F, t693: F, t638: F, t898: F, t643: F, t2142: F, t285: F) -> (F, F, F, F, F, F) {
    let t2149 = t2148 * t698;
    let t2151 = t897 * t27;
    let t2152 = t2151 * t693;
    let t2154 = t638 * t898;
    let t2156 = t643 * t898;
    let t2158 = t2142 * t285;
    (t2149, t2151, t2152, t2154, t2156, t2158)
}
