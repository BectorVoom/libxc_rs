//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 755/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk755(t123: f64, t2281: f64, t868: f64, t2422: f64, t722: f64, t2407: f64, t395: f64, t1808: f64, t2285: f64, t305: f64, t4283: f64, t4284: f64, t4472: f64, t4579: f64, t6104: f64, t6939: f64, t726: f64, t81: f64, t912: f64) -> (f64, f64, f64, f64) {
    let t7126 = t123 * t2281 * t868;
    let t7135 = t123 * t722 * t2422;
    let t7145 = t395 * t2407;
    let t7149 = 0.10611888591559791_f64 * t7126 - 0.06367133154935875_f64 * t123 * t2285 * t868 - 0.06367133154935875_f64 * t123 * t912 * t1808 + 0.053059442957798957_f64 * t7135 - 0.031835665774679375_f64 * t123 * t726 * t2422 - 0.031835665774679375_f64 * t123 * t305 * t6939 - t4283 + 0.31995040645307626_f64 * t4284 + 0.6399008129061525_f64 * t4472 - t4579 - 0.10665013548435875_f64 * t7145 + 0.05332506774217938_f64 * t81 * t6104;
    (t7126, t7135, t7145, t7149)
}
