//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 956/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk956(t2229: f64, t3588: f64, t38: f64, t1234: f64, t2233: f64, t3559: f64, t776: f64, t247: f64, t28: f64, t769: f64, t8276: f64, t3615: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11211 = 70.1526_f64 * t38 * t2229 * t3588;
    let t11222 = 52.61445_f64 * t38 * t2233 * t1234;
    let t11225 = 5.84605_f64 * t38 * t776 * t3559;
    let t11227 = t769 * t28 * t247;
    let t11228 = t8276 * t11227;
    let t11229 = 1.9486833333333333_f64 * t11228;
    let t11230 = t63 * t3615;
    (t11211, t11222, t11225, t11227, t11229, t11230)
}
