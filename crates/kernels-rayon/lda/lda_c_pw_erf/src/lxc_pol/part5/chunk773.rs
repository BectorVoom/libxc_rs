//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 773/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk773(t2627: f64, t411: f64, t156: f64, t2615: f64, t426: f64, t2619: f64, t2624: f64, t431: f64, t325: f64, t128: f64, t6121: f64, t10: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7137 = t2627 * t411;
    let t7142 = t156 * t2615;
    let t7143 = t426 * t7142;
    let t7145 = t156 * t2619;
    let t7146 = t426 * t7145;
    let t7148 = t431 * t2624;
    let t7149 = t7148 * t325;
    let t7151 = t431 * t2627;
    let t7152 = t7151 * t325;
    let t7154 = t128 * t6121;
    let t7155 = t10 * t7154;
    (t7137, t7142, t7143, t7145, t7146, t7148, t7149, t7151, t7152, t7154, t7155)
}
