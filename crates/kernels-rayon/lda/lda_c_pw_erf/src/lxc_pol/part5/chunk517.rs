//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 517/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk517(t143: f64, t2594: f64, t1815: f64, t128: f64, t102: f64, t1558: f64, t2325: f64, t2329: f64, t406: f64, t1563: f64, t2334: f64, t2337: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2595 = t143 * t2594;
    let t2598 = 0.9743416666666667_f64 * t1815;
    let t2599 = t128 * t2594;
    let t2601 = 5.84605_f64 * t102 * t2599;
    let t2602 = t1558 * t2325;
    let t2604 = t406 * t2329;
    let t2606 = t1563 * t2334;
    let t2608 = t408 * t2337;
    (t2595, t2598, t2599, t2601, t2602, t2604, t2606, t2608)
}
