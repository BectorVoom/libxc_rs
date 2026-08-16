//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1029/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1029(t1680: f64, t2026: f64, t432: f64, t4830: f64, t132: f64, t2851: f64, t814: f64, t2852: f64, t802: f64, t3134: f64, t824: f64, t1554: f64, t161: f64, t2100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12227 = t2026 * t1680;
    let t12230 = 2.0_f64 / 15.0_f64 * t432 * t4830;
    let t12232 = t132 * t2851 * t814;
    let t12233 = 4.0_f64 / 405.0_f64 * t12232;
    let t12234 = t802 * t2852;
    let t12235 = 4.0_f64 / 405.0_f64 * t12234;
    let t12237 = t3134 * t824 / 30.0_f64;
    let t12239 = t161 * t1554 * t2100;
    (t12227, t12230, t12233, t12235, t12237, t12239)
}
