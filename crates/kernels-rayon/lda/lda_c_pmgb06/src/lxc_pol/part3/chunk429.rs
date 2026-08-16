//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 429/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk429(t1639: f64, t529: f64, t166: f64, t161: f64, t187: f64, t540: f64, t534: f64, t1553: f64, t1557: f64, t1562: f64, t1590: f64, t1598: f64, t1606: f64, t1633: f64, t1635: f64, t1638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1640 = t1639 * t529;
    let t1641 = t166 * t1640;
    let t1643 = t161 * t1641 / 15.0_f64;
    let t1645 = 8.0_f64 / 3.0_f64 * t540 * t187;
    let t1646 = t534 * t187;
    let t1648 = t1553 - t1557 - t1562 - t1590 + t1598 + t1606 - t1633 - t1635 - t1638 - t1643 + t1645 + 8.0_f64 / 3.0_f64 * t1646;
    (t1640, t1641, t1643, t1645, t1646, t1648)
}
