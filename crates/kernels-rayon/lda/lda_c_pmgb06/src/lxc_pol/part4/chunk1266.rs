//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1266/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1266(t1420: f64, t6475: f64, t12139: f64, t439: f64, t6151: f64, t16608: f64, t16609: f64, t16610: f64, t16613: f64, t16617: f64, t16619: f64, t16621: f64, t16623: f64, t16625: f64, t16629: f64, t16631: f64, t16633: f64, t16635: f64) -> (f64, f64, f64) {
    let t16637 = 16.0_f64 / 81.0_f64 * t1420 * t6475;
    let t16640 = 16.0_f64 / 81.0_f64 * t439 * t12139 * t6151;
    let t16641 = t16608 + t16609 - t16610 + t16613 + t16617 + t16619 + t16621 + t16623 + t16625 + t16629 + t16631 + t16633 + t16635 + t16637 + t16640;
    (t16637, t16640, t16641)
}
