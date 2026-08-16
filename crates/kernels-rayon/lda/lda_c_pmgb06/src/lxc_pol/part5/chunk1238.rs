//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1238/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1238(t13088: f64, t20440: f64, t20442: f64, t20445: f64, t20449: f64, t20451: f64, t20452: f64, t20454: f64, t20456: f64, t20460: f64, t20463: f64, t18274: f64, t18277: f64, t20465: f64, t20467: f64, t20472: f64, t20478: f64, t20480: f64, t20482: f64, t20486: f64, t20490: f64, t20491: f64, t20492: f64) -> (f64, f64) {
    let t21995 = -t20440 + t20442 + t20445 + t20449 + t20451 + t13088 - t20452 - t20454 - t20456 - t20460 + t20463;
    let t21997 = -t20465 - t20467 - t20472 + t20478 - t20480 + t20482 - t20486 + t20490 - t20491 - t20492 + t18274 + 0.18233333333333332_f64 * t18277;
    (t21995, t21997)
}
