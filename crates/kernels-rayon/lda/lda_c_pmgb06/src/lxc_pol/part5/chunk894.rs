//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 894/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk894(t113: f64, t301: f64, t3951: f64, t83: f64, t3993: f64, t413: f64, t2789: f64, t718: f64, t1135: f64, t1183: f64, t1139: f64, t1100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10603 = 1.0943113336969376e-06_f64 * t3951 * t83 * t113 * t301;
    let t10609 = t3993 * t413 * t301;
    let t10614 = 0.0011622696607154768_f64 * t718 * t2789 * t301;
    let t10617 = 0.008135887625008338_f64 * t1135 * t1183 * t301;
    let t10635 = t1139 * t1183 * t301;
    let t10637 = t1100 * t83;
    (t10603, t10609, t10614, t10617, t10635, t10637)
}
