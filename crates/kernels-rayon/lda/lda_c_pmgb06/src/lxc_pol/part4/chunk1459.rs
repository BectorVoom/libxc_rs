//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1459/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1459(t1271: f64, t2715: f64, t955: f64, t1238: f64, t2696: f64, t348: f64, t350: f64, t7015: f64, t2699: f64, t18568: f64, t35: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t18725 = t1271 * t2715 * t955;
    let t18728 = t1238 * t2696 * t955;
    let t18729 = 0.6495611111111111_f64 * t18728;
    let t18731 = t348 * t7015 * t350;
    let t18732 = 0.9743416666666667_f64 * t18731;
    let t18734 = t1238 * t2699 * t955;
    let t18735 = 0.3247805555555556_f64 * t18734;
    let t18737 = t35 * t64 * t18568;
    (t18725, t18729, t18732, t18735, t18737)
}
