//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 447/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk447(t1730: f64, t206: f64, t1562: f64, t1590: f64, t1598: f64, t1606: f64, t1633: f64, t1635: f64, t1638: f64, t1643: f64, t1708: f64, t1712: f64, t1727: f64, t224: f64) -> (f64, f64) {
    let t1732 = 0.033245444444444446_f64 * t206 * t1730;
    let t1733 = -t1562 - 4.0_f64 / 45.0_f64 * t1708 + t1712 - t1727 * t224 / 15.0_f64 - t1590 + t1598 + t1606 - t1633 - t1635 - t1638 - t1643 + t1732;
    (t1732, t1733)
}
