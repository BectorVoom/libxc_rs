//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 716/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk716(t1395: f64, t2648: f64, t137: f64, t132: f64, t2043: f64, t802: f64, t2066: f64, t2650: f64, t432: f64, t2625: f64, t486: f64, t1639: f64, t2623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6583 = t1395 * t2648;
    let t6584 = t137 * t6583;
    let t6586 = t132 * t6584 / 30.0_f64;
    let t6588 = t802 * t2043 / 15.0_f64;
    let t6590 = t802 * t2066 / 15.0_f64;
    let t6592 = t432 * t2650 / 30.0_f64;
    let t6594 = t486 * t2625 / 30.0_f64;
    let t6595 = t1639 * t2623;
    (t6583, t6584, t6586, t6588, t6590, t6592, t6594, t6595)
}
