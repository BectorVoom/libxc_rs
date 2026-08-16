//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 796/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk796(t2065: f64, t435: f64, t132: f64, t2015: f64, t432: f64, t1596: f64, t802: f64, t1915: f64, t4861: f64, t493: f64, t1602: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5326 = t435 * t2065;
    let t5328 = 2.0_f64 / 45.0_f64 * t132 * t5326;
    let t5330 = 2.0_f64 / 45.0_f64 * t432 * t2015;
    let t5332 = t802 * t1596 / 15.0_f64;
    let t5333 = t1915 * t4861;
    let t5335 = 2.0_f64 / 15.0_f64 * t493 * t5333;
    let t5336 = t838 * t1602;
    (t5326, t5328, t5330, t5332, t5333, t5335, t5336)
}
