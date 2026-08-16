//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1053(t242: f64, t6138: f64, t2594: f64, t2765: f64, t440: f64, t7199: f64, t7191: f64, t7158: f64, t925: f64, t7161: f64, t1686: f64, t2624: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19397 = t6138 * t242;
    let t19421 = t2765 * t2594 * t440;
    let t19425 = t2765 * t7199;
    let t19449 = t2765 * t7191;
    let t19516 = t7158 * t925;
    let t19518 = t7161 * t925;
    let t19523 = t1686 * t2624 * t933;
    (t19397, t19421, t19425, t19449, t19516, t19518, t19523)
}
