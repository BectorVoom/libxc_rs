//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 798/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk798(t2592: f64, t815: f64, t2584: f64, t802: f64, t3092: f64, t7284: f64, t3090: f64, t36: f64, t3098: f64, t1525: f64, t1858: f64, t2381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7470 = t2592 * t815 / 10.0_f64;
    let t7472 = t802 * t2584 / 10.0_f64;
    let t7477 = t3092 * t7284;
    let t7478 = t3090 * t7477;
    let t7479 = t36 * t7478;
    let t7481 = t3098 * t7284;
    let t7482 = t1525 * t7481;
    let t7483 = t36 * t7482;
    let t7485 = t1858 * t2381;
    (t7470, t7472, t7477, t7478, t7479, t7481, t7482, t7483, t7485)
}
