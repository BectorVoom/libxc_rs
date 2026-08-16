//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 682/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk682(t4238: f64, t83: f64, t419: f64, t1770: f64, t1767: f64, t398: f64, t1186: f64, t1768: f64, t123: f64, t199: f64, t2822: f64, t2833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4239 = t4238 * t83;
    let t4240 = t4239 * t419;
    let t4242 = 0.0001639671923854359_f64 * t4240 * t1770;
    let t4243 = t1767 * t398;
    let t4244 = t4243 * t419;
    let t4245 = t4244 * t1770;
    let t4247 = t1768 * t1186;
    let t4249 = 5.4655730795145296e-05_f64 * t4247 * t1770;
    let t4252 = 0.5188034422540342_f64 * t123 * t2822 * t199;
    let t4254 = t123 * t2833 * t199;
    (t4239, t4240, t4242, t4243, t4244, t4245, t4247, t4249, t4252, t4254)
}
