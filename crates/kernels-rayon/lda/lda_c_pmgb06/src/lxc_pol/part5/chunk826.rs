//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 826/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk826(t3358: f64, t7594: f64, t525: f64, t7612: f64, t1576: f64, t7598: f64, t7516: f64, t7605: f64, t7512: f64, t103: f64, t4911: f64, t7600: f64, t7603: f64, t7607: f64, t7610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7834 = t3358 * t7594;
    let t7837 = t525 * t7612;
    let t7844 = t1576 * t7598;
    let t7847 = t1576 * t7516;
    let t7850 = t525 * t7605;
    let t7853 = t525 * t7512;
    let t7856 = -0.047988888888888886_f64 * t4911 - 0.002962962962962963_f64 * t103 * t7834 - 0.006666666666666667_f64 * t103 * t7837 + 0.14396666666666666_f64 * t7600 - 0.07198333333333333_f64 * t7603 - 0.21595_f64 * t7607 + 0.21595_f64 * t7610 + 0.013333333333333334_f64 * t103 * t7844 - 0.006666666666666667_f64 * t103 * t7847 - 0.04_f64 * t103 * t7850 + 0.04_f64 * t103 * t7853;
    (t7834, t7837, t7844, t7847, t7850, t7853, t7856)
}
