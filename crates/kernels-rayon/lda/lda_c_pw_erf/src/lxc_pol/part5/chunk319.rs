//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 319/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk319(t1138: f64, t20: f64, t161: f64, t635: f64, t1129: f64, t1132: f64, t1135: f64, t628: f64, t629: f64) -> (f64, f64, f64) {
    let t1139 = t1138 * t20;
    let t1140 = t635 * t161;
    let t1143 = t1129 / 2.0_f64 + 0.0627_f64 * t1132 * t629 - 0.0418_f64 * t628 * t1135 + 0.00786258_f64 * t1139 * t1140;
    (t1139, t1140, t1143)
}
