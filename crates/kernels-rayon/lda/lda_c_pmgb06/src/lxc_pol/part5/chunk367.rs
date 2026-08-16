//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 367/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk367(t489: f64, t530: f64, t161: f64, t511: f64, t517: f64, t187: f64, t540: f64, t534: f64, t199: f64, t718: f64, t1329: f64, t391: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1636 = t489 * t530;
    let t1637 = t161 * t1636;
    let t1639 = t511 * t517;
    let t1645 = 8.0_f64 / 3.0_f64 * t540 * t187;
    let t1646 = t534 * t187;
    let t1658 = 0.1675256410710088_f64 * t718 * t199;
    let t1659 = t1329 * t199;
    let t1661 = t391 * t566;
    (t1636, t1637, t1639, t1645, t1646, t1658, t1659, t1661)
}
