//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 766/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk766(t187: f64, t2676: f64, t5186: f64, t5196: f64, t5207: f64, t5209: f64, t6540: f64, t6543: f64, t6547: f64, t6549: f64, t6553: f64, t6558: f64, t6564: f64, t6565: f64, t6566: f64, t6567: f64) -> (f64, f64) {
    let t7205 = t2676 * t187;
    let t7207 = t6540 + t6543 + t6547 + t6549 + t6553 + t6558 + t6564 - t6565 - t6566 - t6567 + 4.0_f64 / 3.0_f64 * t7205 + t5186 + t5196 + t5207 + t5209;
    (t7205, t7207)
}
