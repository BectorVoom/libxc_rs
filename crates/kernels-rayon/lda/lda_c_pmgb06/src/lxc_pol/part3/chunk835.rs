//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 835/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk835(t328: f64, t4329: f64, t1777: f64, t754: f64, t936: f64, t97: f64, t1786: f64, t27: f64, t2767: f64, t749: f64, t2760: f64, t2771: f64) -> (f64, f64, f64, f64) {
    let t8024 = t4329 * t328;
    let t8028 = t1777 * t754 * t97 * t936;
    let t8032 = t749 * t1786 * t27 * t2767;
    let t8034 = t2760 * t2771;
    (t8024, t8028, t8032, t8034)
}
