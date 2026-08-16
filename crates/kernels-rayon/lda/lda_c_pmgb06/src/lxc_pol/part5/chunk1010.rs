//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1010/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1010(t10548: f64, t73: f64, t26: f64, t2732: f64, t329: f64, t2407: f64, t247: f64, t1156: f64, t123: f64, t2422: f64, t395: f64, t6104: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18926 = t10548 * t73;
    let t18939 = t26 * t2732;
    let t18940 = t329 * t18939;
    let t18954 = t247 * t2407;
    let t18969 = t123 * t1156 * t2422;
    let t18979 = t395 * t6104;
    (t18926, t18939, t18940, t18954, t18969, t18979)
}
