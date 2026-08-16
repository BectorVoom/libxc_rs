//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 924/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk924(t2789: f64, t301: f64, t718: f64, t1135: f64, t1183: f64, t113: f64, t2803: f64, t395: f64, t3982: f64, t413: f64, t26: f64, t4038: f64) -> (f64, f64, f64, f64, f64) {
    let t10614 = 0.0011622696607154768_f64 * t718 * t2789 * t301;
    let t10617 = 0.008135887625008338_f64 * t1135 * t1183 * t301;
    let t10620 = t395 * t2803 * t113 * t301;
    let t10623 = t3982 * t413 * t301;
    let t10625 = t4038 * t26;
    (t10614, t10617, t10620, t10623, t10625)
}
