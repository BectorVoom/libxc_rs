//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 624/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk624(t2061: f64, t4913: f64, t2057: f64, t405: f64, t2054: f64, t1554: f64, t843: f64, t161: f64, t1555: f64, t831: f64, t1548: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5006 = t4913 * t2061;
    let t5032 = 0.017777777777777778_f64 * t405 * t2057;
    let t5034 = 0.002962962962962963_f64 * t405 * t2054;
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5047 = t831 * t1555;
    let t5049 = t802 * t1548;
    (t5006, t5032, t5034, t5044, t5045, t5047, t5049)
}
