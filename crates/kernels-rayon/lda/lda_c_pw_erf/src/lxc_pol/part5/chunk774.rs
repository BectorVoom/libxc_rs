//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 774/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk774(t2599: f64, t415: f64, t325: f64, t2611: f64, t3313: f64, t3322: f64, t426: f64, t5598: f64, t5609: f64, t7143: f64, t7146: f64, t7149: f64, t7152: f64, t7155: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7158 = t415 * t2599;
    let t7159 = t7158 * t325;
    let t7160 = 0.9743416666666667_f64 * t7159;
    let t7161 = t415 * t2611;
    let t7162 = t7161 * t325;
    let t7163 = 0.48717083333333333_f64 * t7162;
    let t7164 = -t5598 - t5609 - t7143 / 2.0_f64 + t7146 / 6.0_f64 - 2.93808_f64 * t7149 + 0.73452_f64 * t7152 - t426 * t7155 / 2.0_f64 - t7160 + t7163 + t3313 - t3322;
    (t7158, t7159, t7160, t7161, t7162, t7163, t7164)
}
