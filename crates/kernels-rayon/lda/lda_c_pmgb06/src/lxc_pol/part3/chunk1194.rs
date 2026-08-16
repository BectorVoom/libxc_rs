//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1194/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1194(t11972: f64, t11974: f64, t11977: f64, t11981: f64, t11985: f64, t11987: f64, t11990: f64, t11994: f64, t12000: f64, t12003: f64, t12005: f64, t12011: f64, t12015: f64, t12017: f64, t12019: f64, t12021: f64, t12023: f64, t12026: f64, t12028: f64, t12032: f64, t12035: f64, t12038: f64, t12040: f64) -> (f64, f64) {
    let t14335 = -t11972 + t11974 + t11977 - t11981 - t11985 + t11987 + t11990 + t11994 - t12000 + t12003 - t12005;
    let t14336 = -t12011 - t12015 - t12017 - t12019 - t12021 + t12023 + t12026 + t12028 + t12032 + t12035 - t12038 - t12040;
    (t14335, t14336)
}
