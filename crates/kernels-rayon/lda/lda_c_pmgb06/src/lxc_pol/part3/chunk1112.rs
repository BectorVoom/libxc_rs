//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1112/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1112(t13215: f64, t2012: f64, t431: f64, t5210: f64, t1423: f64, t5171: f64, t1631: f64, t1887: f64, t3047: f64, t802: f64, t10040: f64, t10046: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13216 = 2.0_f64 / 45.0_f64 * t13215;
    let t13218 = t431 * t5210 * t2012;
    let t13219 = 2.0_f64 / 9.0_f64 * t13218;
    let t13220 = t1423 * t5171;
    let t13221 = 2.0_f64 / 45.0_f64 * t13220;
    let t13223 = t1887 * t1631 / 10.0_f64;
    let t13225 = t802 * t3047 / 10.0_f64;
    let t13226 = t10040 / 15.0_f64;
    let t13227 = t10046 / 45.0_f64;
    (t13216, t13219, t13221, t13223, t13225, t13226, t13227)
}
