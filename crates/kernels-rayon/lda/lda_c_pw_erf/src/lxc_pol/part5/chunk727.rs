//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 727/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk727(t503: f64, t6492: f64, t11: f64, t6442: f64, t1953: f64, t6331: f64, t1243: f64, t6336: f64, t2434: f64, t325: f64, t2438: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6532 = t503 * t6492;
    let t6533 = t11 * t6532;
    let t6535 = t503 * t6442;
    let t6536 = t1953 * t6535;
    let t6538 = t503 * t6331;
    let t6539 = t11 * t6538;
    let t6541 = t1243 * t6336;
    let t6542 = t11 * t6541;
    let t6545 = t325 * t2434;
    let t6547 = t325 * t2438;
    let t6549 = t325 * t2430;
    (t6532, t6533, t6535, t6536, t6538, t6539, t6541, t6542, t6545, t6547, t6549)
}
