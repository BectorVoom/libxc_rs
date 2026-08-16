//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 918/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk918(t197: f64, t3783: f64, t529: f64, t4048: f64, t9: f64, t3892: f64, t1245: f64, t187: f64, t22: f64, t1484: f64, t155: f64, t219: f64, t3762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10463 = t3783 * t197;
    let t10467 = t3783 * t529;
    let t10527 = t9 * t4048;
    let t10557 = t9 * t3892;
    let t10567 = t22 / t187 / t1245;
    let t10605 = t155 * t1484;
    let t10654 = t3762 * t219;
    (t10463, t10467, t10527, t10557, t10567, t10605, t10654)
}
