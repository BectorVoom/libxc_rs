//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1015/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1015(t9501: f64, t139: f64, t3247: f64, t1463: f64, t1413: f64, t1486: f64, t947: f64, t1478: f64, t1830: f64, t508: f64, t1482: f64, t132: f64, t2851: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9502 = 0.01959135802469136_f64 * t9501;
    let t9507 = t139 * t3247;
    let t9508 = t1463 * t1463;
    let t9509 = 1.0_f64 / t9508;
    let t9525 = 1.0_f64 / t1463 / t1413;
    let t9530 = t947 * t1486;
    let t9532 = t947 * t1478;
    let t9552 = t1830 * t508;
    let t9577 = t947 * t1482;
    let t9596 = t132 * t2851 * t478;
    (t9502, t9507, t9509, t9525, t9530, t9532, t9552, t9577, t9596)
}
