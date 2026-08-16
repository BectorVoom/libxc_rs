//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 896/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk896(t9508: f64, t2920: f64, t350: f64, t1413: f64, t1463: f64, t1486: f64, t947: f64, t1478: f64, t2940: f64, t2914: f64, t1830: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9509 = 1.0_f64 / t9508;
    let t9522 = t350 * t2920;
    let t9525 = 1.0_f64 / t1463 / t1413;
    let t9530 = t947 * t1486;
    let t9532 = t947 * t1478;
    let t9534 = t350 * t2940;
    let t9537 = t350 * t2914;
    let t9552 = t1830 * t508;
    (t9509, t9522, t9525, t9530, t9532, t9534, t9537, t9552)
}
