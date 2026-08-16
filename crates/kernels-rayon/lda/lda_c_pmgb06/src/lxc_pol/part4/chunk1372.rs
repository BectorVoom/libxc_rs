//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1372/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1372(t441: f64, t6673: f64, t439: f64, t445: f64, t224: f64, t6687: f64, t500: f64, t1451: f64, t6134: f64, t17990: f64, t17992: f64, t17994: f64, t17997: f64, t18001: f64, t18003: f64, t18005: f64, t18007: f64, t18009: f64, t18011: f64, t18013: f64, t18015: f64) -> (f64, f64, f64, f64) {
    let t18016 = t441 * t6673;
    let t18019 = 2.0_f64 / 45.0_f64 * t439 * t18016 * t445;
    let t18020 = t6687 * t224;
    let t18022 = 2.0_f64 / 45.0_f64 * t18020 * t500;
    let t18024 = 2.0_f64 / 45.0_f64 * t6134 * t1451;
    let t18025 = t17990 + t17992 + t17994 - t17997 - t18001 + t18003 + t18005 + t18007 - t18009 - t18011 + t18013 + t18015 + t18019 + t18022 + t18024;
    (t18019, t18022, t18024, t18025)
}
