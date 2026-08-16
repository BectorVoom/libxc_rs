//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 821/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk821(t3615: f64, t769: f64, t4394: f64, t56: f64, t38: f64, t370: f64, t1234: f64, t2229: f64, t365: f64, t110: f64, t30: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5756 = t3615 * t769;
    let t5760 = t56 * t4394;
    let t5762 = 2.923025_f64 * t38 * t5760;
    let t5763 = t370 * t4394;
    let t5766 = t2229 * t1234;
    let t5770 = t365 * t2229;
    let t5772 = t30 * t110 * t342;
    (t5756, t5760, t5762, t5763, t5766, t5770, t5772)
}
