//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 654/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk654(t545: f64, t5655: f64, t1366: f64, t2349: f64, t187: f64, t2342: f64, t2345: f64, t1799: f64, t415: f64, t1347: f64, t795: f64, t118: f64, t5522: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5656 = t5655 * t545;
    let t5658 = t2349 * t1366;
    let t5674 = 8.0_f64 / 3.0_f64 * t2342 * t187;
    let t5675 = t2345 * t187;
    let t5697 = 0.06301081444628223_f64 * t1799 * t415;
    let t5698 = t795 * t1347;
    let t5701 = 0.06301081444628223_f64 * t5522 * t118;
    (t5656, t5658, t5674, t5675, t5697, t5698, t5701)
}
