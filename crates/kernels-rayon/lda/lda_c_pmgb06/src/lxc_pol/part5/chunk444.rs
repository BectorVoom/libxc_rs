//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 444/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk444(t2148: f64, t698: f64, t27: f64, t897: f64, t693: f64, t638: f64, t898: f64, t643: f64, t2142: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2149 = t2148 * t698;
    let t2151 = t897 * t27;
    let t2152 = t2151 * t693;
    let t2154 = t638 * t898;
    let t2156 = t643 * t898;
    let t2158 = t2142 * t285;
    (t2149, t2151, t2152, t2154, t2156, t2158)
}
