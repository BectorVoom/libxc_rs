//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 727/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk727(t2088: f64, t2093: f64, t166: f64, t161: f64, t2582: f64, t464: f64, t477: f64, t137: f64, t132: f64, t2592: f64, t479: f64, t1912: f64, t1972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6730 = t2093 * t2088;
    let t6731 = t166 * t6730;
    let t6733 = t161 * t6731 / 15.0_f64;
    let t6734 = t2582 * t464;
    let t6735 = t6734 * t477;
    let t6736 = t137 * t6735;
    let t6738 = t132 * t6736 / 30.0_f64;
    let t6740 = t2592 * t479 / 30.0_f64;
    let t6743 = 2.0_f64 / 45.0_f64 * t1972 * t1912;
    (t6730, t6731, t6733, t6734, t6735, t6736, t6738, t6740, t6743)
}
