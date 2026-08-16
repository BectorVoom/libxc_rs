//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1054/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1054(t1686: f64, t2627: f64, t933: f64, t2615: f64, t474: f64, t426: f64, t2619: f64, t7148: f64, t925: f64, t7151: f64, t325: f64, t431: f64, t7123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19526 = t1686 * t2627 * t933;
    let t19532 = t474 * t2615;
    let t19533 = t426 * t19532;
    let t19539 = t474 * t2619;
    let t19540 = t426 * t19539;
    let t19544 = t7148 * t925;
    let t19546 = t7151 * t925;
    let t19549 = t431 * t7123 * t325;
    (t19526, t19532, t19533, t19539, t19540, t19544, t19546, t19549)
}
