//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1311/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1311(t161: f64, t166: f64, t17047: f64, t17077: f64, t17097: f64, t17121: f64, t17159: f64, t17195: f64, t17229: f64, t17245: f64, t518: f64, t10046: f64) -> (f64, f64) {
    let t17252 = t161 * t166 * t518 * (t17047 + t17077 + t17097 + t17121 + t17159 + t17195 + t17229 + t17245) / 30.0_f64;
    let t17253 = t10046 / 135.0_f64;
    (t17252, t17253)
}
