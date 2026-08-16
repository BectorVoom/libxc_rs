//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 412/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk412(t132: f64, t1548: f64, t1469: f64, t1493: f64, t1501: f64, t1503: f64, t1506: f64, t1514: f64, t1516: f64, t1519: f64, t1544: f64, t1546: f64) -> (f64, f64) {
    let t1550 = t132 * t1548 / 135.0_f64;
    let t1551 = t1469 + t1493 + t1501 + t1503 + t1506 + t1514 + t1516 + t1519 + t1544 + t1546 - t1550;
    (t1550, t1551)
}
