//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 915/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk915(t2176: f64, t524: f64, t519: f64, t1155: f64, t603: f64, t10042: f64, t2061: f64, t590: f64, t1333: f64, t191: f64, t205: f64, t190: f64, t212: f64, t9821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10166 = t2176 * t524;
    let t10167 = t519 * t10166;
    let t10172 = 0.004413481481481482_f64 * t1155 * t603;
    let t10195 = 0.3732469135802469_f64 * t10042;
    let t10202 = t2061 * t590;
    let t10216 = t191 / t205 / t1333;
    let t10225 = 0.10864197530864197_f64 * t190 * t9821 * t212;
    (t10167, t10172, t10195, t10202, t10216, t10225)
}
