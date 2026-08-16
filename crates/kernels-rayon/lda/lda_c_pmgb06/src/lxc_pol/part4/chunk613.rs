//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 613/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk613(t1138: f64, t1145: f64, t1151: f64, t1753: f64, t1765: f64, t1772: f64, t1775: f64, t2172: f64, t2176: f64, t2267: f64, t2269: f64, t2747: f64) -> f64 {
    let t2752 = -0.02394846802050922_f64 * t2267 - 0.0005811348303577384_f64 * t2176 + 0.039914113367515366_f64 * t2269 - 0.10809180959278285_f64 * t2172 + t1138 - t1145 + t1151 + t1753 - t1765 - t1772 - t1775;
    let t2753 = t2747 + t2752;
    t2753
}
