//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1001/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1001(t2089: f64, t933: f64, t2954: f64, t4609: f64, t11: f64, t1243: f64, t1971: f64, t2961: f64, t503: f64, t4632: f64, t945: f64, t1953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11695 = t933 * t2089;
    let t11697 = t4609 * t2954;
    let t11699 = t11 * t1243 * t11697;
    let t11701 = t1971 * t2961;
    let t11703 = t11 * t503 * t11701;
    let t11705 = t4632 * t945;
    let t11707 = t1953 * t503 * t11705;
    (t11695, t11697, t11699, t11701, t11703, t11705, t11707)
}
