//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 965/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk965(t1423: f64, t6495: f64, t6499: f64, t405: f64, t6193: f64, t6147: f64, t4913: f64, t6196: f64, t6199: f64, t6202: f64, t2642: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15521 = t1423 * t6495;
    let t15523 = t1423 * t6499;
    let t15589 = t405 * t6193;
    let t15591 = t405 * t6147;
    let t15593 = t4913 * t6196;
    let t15601 = t405 * t6199;
    let t15603 = t405 * t6202;
    let t15644 = t955 * t2642;
    (t15521, t15523, t15589, t15591, t15593, t15601, t15603, t15644)
}
