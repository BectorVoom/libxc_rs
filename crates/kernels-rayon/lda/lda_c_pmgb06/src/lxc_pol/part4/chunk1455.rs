//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1455/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1455(t11335: f64, t11343: f64, t2209: f64, t2249: f64, t5874: f64, t1227: f64, t2707: f64, t38: f64, t11320: f64, t11322: f64, t11330: f64, t11341: f64, t11354: f64, t11357: f64, t11364: f64, t11407: f64, t2229: f64, t4394: f64, t63: f64, t6989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18644 = 5.84605_f64 * t11335;
    let t18646 = 1.9486833333333333_f64 * t11343;
    let t18649 = t2249 * t2209;
    let t18650 = t5874 * t18649;
    let t18656 = 5.84605_f64 * t38 * t2707 * t1227;
    let t18663 = 4.0_f64 * t11320 + 15.66976_f64 * t11322 - t11330 + t18644 - 5.87616_f64 * t11341 - t18646 + 8.0_f64 / 3.0_f64 * t11354 - 11.75232_f64 * t11357 - 24.0_f64 * t11407 * t18650 + 29.3808_f64 * t11364 + t18656 - 29.3808_f64 * t63 * t6989 * t1227 + 11.75232_f64 * t63 * t2229 * t4394;
    (t18644, t18646, t18649, t18650, t18656, t18663)
}
