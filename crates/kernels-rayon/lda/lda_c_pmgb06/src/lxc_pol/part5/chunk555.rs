//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 555/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk555(t1554: f64, t512: f64, t161: f64, t176: f64, t2918: f64, t153: f64, t3098: f64, t129: f64, t1710: f64) -> (f64, f64, f64, f64, f64) {
    let t3155 = t1554 * t512;
    let t3156 = t161 * t3155;
    let t3172 = t176 * t2918;
    let t3189 = t153 * t3098;
    let t3213 = t129 * t1710;
    (t3155, t3156, t3172, t3189, t3213)
}
