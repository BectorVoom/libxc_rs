//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 826/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk826<F: Float>(t3358: F, t7594: F, t525: F, t7612: F, t1576: F, t7598: F, t7516: F, t7605: F, t7512: F, t103: F, t4911: F, t7600: F, t7603: F, t7607: F, t7610: F) -> (F, F, F, F, F, F, F) {
    let t7834 = t3358 * t7594;
    let t7837 = t525 * t7612;
    let t7844 = t1576 * t7598;
    let t7847 = t1576 * t7516;
    let t7850 = t525 * t7605;
    let t7853 = t525 * t7512;
    let t7856 = -F::new(0.047988888888888886) * t4911 - F::new(0.002962962962962963) * t103 * t7834 - F::new(0.006666666666666667) * t103 * t7837 + F::new(0.14396666666666666) * t7600 - F::new(0.07198333333333333) * t7603 - F::new(0.21595) * t7607 + F::new(0.21595) * t7610 + F::new(0.013333333333333334) * t103 * t7844 - F::new(0.006666666666666667) * t103 * t7847 - F::new(0.04) * t103 * t7850 + F::new(0.04) * t103 * t7853;
    (t7834, t7837, t7844, t7847, t7850, t7853, t7856)
}
