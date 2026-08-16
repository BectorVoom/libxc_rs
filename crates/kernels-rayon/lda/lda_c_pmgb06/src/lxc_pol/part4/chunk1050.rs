//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1050/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1050(t1105: f64, t2158: f64, t2148: f64, t3729: f64, t27: f64, t4515: f64, t693: f64, t3725: f64, t1108: f64, t2160: f64, t1112: f64, t4529: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11139 = t1105 * t2158;
    let t11142 = t2148 * t3729;
    let t11145 = t4515 * t27 * t693;
    let t11147 = t2148 * t3725;
    let t11149 = t1108 * t2160;
    let t11155 = t4529 * t1112;
    (t11139, t11142, t11145, t11147, t11149, t11155)
}
