//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1116/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1116(t4906: f64, t4913: f64, t1464: f64, t524: f64, t2911: f64, t3357: f64, t146: f64, t4918: f64, t9712: f64, t1575: f64, t2918: f64, t13560: f64, t2085: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14082 = t4913 * t4906;
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    let t14150 = t146 * t9712 * t4918;
    let t14152 = t1575 * t2918;
    let t14162 = t13560 * t2085;
    (t14082, t14106, t14110, t14150, t14152, t14162)
}
