//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 886/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk886(t3146: f64, t490: f64, t1490: f64, t1554: f64, t161: f64, t132: f64, t1541: f64, t1547: f64, t1710: f64, t485: f64, t500: f64, t1451: f64, t3223: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9239 = t3146 * t490;
    let t9242 = t161 * t1554 * t1490;
    let t9259 = t132 * t1547 * t1541;
    let t9266 = t485 * t1710;
    let t9267 = t9266 * t500;
    let t9269 = t3223 * t1451;
    (t9239, t9242, t9259, t9266, t9267, t9269)
}
