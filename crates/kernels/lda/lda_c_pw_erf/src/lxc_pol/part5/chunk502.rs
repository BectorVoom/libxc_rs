//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 502/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk502<F: Float>(t143: F, t2594: F, t1815: F, t128: F, t102: F, t1558: F, t2325: F, t2329: F, t406: F, t1563: F, t2334: F, t2337: F, t408: F) -> (F, F, F, F, F) {
    let t2595 = t143 * t2594;
    let t2598 = 0.9743416666666667 * t1815;
    let t2599 = t128 * t2594;
    let t2601 = 5.84605 * t102 * t2599;
    let t2602 = t1558 * t2325;
    let t2604 = t406 * t2329;
    let t2606 = t1563 * t2334;
    let t2608 = t408 * t2337;
    let t2610 = -t2602 / 9.0 + t2604 / 3.0 - t2606 / 9.0 + t2608 / 3.0;
    (t2595, t2598, t2599, t2601, t2610)
}
