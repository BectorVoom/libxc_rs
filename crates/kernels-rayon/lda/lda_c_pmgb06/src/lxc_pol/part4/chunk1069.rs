//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1069/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1069(t1989: f64, t3226: f64, t2007: f64, t3213: f64, t131: f64, t1767: f64, t129: f64, t2012: f64, t10318: f64, t806: f64, t436: f64, t4754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11832 = t3226 * t1989;
    let t11860 = t3213 * t2007;
    let t11862 = t131 * t1767;
    let t11864 = t129 * t11862 * t2012;
    let t11866 = t10318 * t806;
    let t11868 = t4754 * t436;
    (t11832, t11860, t11862, t11864, t11866, t11868)
}
