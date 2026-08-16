//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 415/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk415(t1069: f64, t1531: f64, t453: f64, t36: f64, t1074: f64, t443: f64, t1522: f64, t1523: f64, t1528: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1532 = t1531 * t1069;
    let t1533 = t453 * t1532;
    let t1534 = t36 * t1533;
    let t1536 = t443 * t1074;
    let t1537 = t453 * t1536;
    let t1538 = t36 * t1537;
    let t1540 = -t1522 - 0.0012594444444444445_f64 * t1523 + 0.0012594444444444445_f64 * t1528 - 0.003778333333333333_f64 * t1534 + 0.0018891666666666666_f64 * t1538;
    (t1532, t1533, t1534, t1536, t1537, t1538, t1540)
}
