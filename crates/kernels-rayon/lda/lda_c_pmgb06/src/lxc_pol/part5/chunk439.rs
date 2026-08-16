//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 439/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk439(t2106: f64, t477: f64, t137: f64, t132: f64, t1552: f64, t1637: f64, t1550: f64, t1557: f64, t1708: f64, t1712: f64, t1732: f64, t2039: f64, t2041: f64, t2045: f64, t2068: f64, t2070: f64, t2092: f64, t2097: f64, t2099: f64, t2103: f64, t2105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2107 = t2106 * t477;
    let t2108 = t137 * t2107;
    let t2110 = t132 * t2108 / 30.0_f64;
    let t2111 = t1552 / 45.0_f64;
    let t2113 = t1637 / 45.0_f64;
    let t2114 = t2039 - t1550 - t2041 - t2045 - t2068 - t2070 - t2092 - t2097 - t2099 - t2103 - t2105 - t2110 + t2111 - t1557 - 2.0_f64 / 45.0_f64 * t1708 + t1712 - t2113 + t1732;
    (t2107, t2108, t2110, t2111, t2113, t2114)
}
