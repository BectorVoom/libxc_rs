//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 191/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk191(t507: f64, t525: f64, t103: f64, t509: f64, t519: f64, t523: f64) -> (f64, f64) {
    let t526 = t525 * t507;
    let t529 = -t519 - 0.035991666666666665_f64 * t509 - t523 - 0.006666666666666667_f64 * t103 * t526;
    (t526, t529)
}
