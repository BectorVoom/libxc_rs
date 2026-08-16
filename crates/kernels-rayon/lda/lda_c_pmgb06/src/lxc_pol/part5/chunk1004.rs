//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1004/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1004(t365: f64, t5772: f64, t6996: f64, t2703: f64, t348: f64, t110: f64, t2209: f64, t30: f64, t5783: f64, t360: f64, t7031: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18582 = t365 * t6996 * t5772;
    let t18585 = t348 * t2703 * t5772;
    let t18588 = t30 * t110 * t2209;
    let t18589 = t5783 * t18588;
    let t18609 = t360 * t110 * t7031;
    let t18615 = t348 * t2707 * t5772;
    (t18582, t18585, t18588, t18589, t18609, t18615)
}
