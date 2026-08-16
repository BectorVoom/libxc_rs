//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 858/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk858(t374: f64, t783: f64, t6007: f64, t342: f64, t4232: f64, t1233: f64, t5582: f64, t1322: f64, t2732: f64, t787: f64, t2695: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6008 = t783 * t374;
    let t6009 = t6007 * t6008;
    let t6012 = t783 * t342;
    let t6013 = t4232 * t6012;
    let t6018 = t1233 * t5582;
    let t6021 = t2732 * t1322;
    let t6024 = t787 * t342;
    let t6028 = t787 * t374;
    let t6031 = t73 * t2695;
    (t6008, t6009, t6012, t6013, t6018, t6021, t6024, t6028, t6031)
}
