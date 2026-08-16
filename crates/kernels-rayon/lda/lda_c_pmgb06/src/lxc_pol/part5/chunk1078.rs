//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1078/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1078(t132: f64, t137: f64, t16267: f64, t822: f64, t1887: f64, t2584: f64, t16284: f64, t16286: f64, t16294: f64, t12240: f64, t12246: f64, t19954: f64, t19956: f64, t19958: f64, t19960: f64, t19963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19967 = t132 * t137 * t16267 * t822 / 10.0_f64;
    let t19969 = t1887 * t2584 / 10.0_f64;
    let t19970 = 2.0_f64 / 15.0_f64 * t16284;
    let t19971 = 2.0_f64 / 15.0_f64 * t16286;
    let t19972 = 2.0_f64 / 15.0_f64 * t16294;
    let t19973 = t19954 - t19956 + t19958 + t19960 + t19963 + t12240 + t12246 - t19967 + t19969 - t19970 - t19971 - t19972;
    (t19967, t19969, t19970, t19971, t19972, t19973)
}
