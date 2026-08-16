//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 823/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk823(t473: f64, t7493: f64, t1619: f64, t7481: f64, t7485: f64, t3404: f64, t7477: f64, t7497: f64, t103: f64, t7479: f64, t7483: f64, t7487: f64, t7491: f64, t7495: f64, t7499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7779 = t473 * t7493;
    let t7782 = t1619 * t7481;
    let t7785 = t1619 * t7485;
    let t7788 = t3404 * t7477;
    let t7791 = t473 * t7497;
    let t7800 = 0.04_f64 * t103 * t7779 + 0.013333333333333334_f64 * t103 * t7782 - 0.006666666666666667_f64 * t103 * t7785 - 0.002962962962962963_f64 * t103 * t7788 - 0.006666666666666667_f64 * t103 * t7791 - 0.03999074074074074_f64 * t7479 - 0.035991666666666665_f64 * t7499 + 0.14396666666666666_f64 * t7483 - 0.07198333333333333_f64 * t7487 - 0.21595_f64 * t7491 + 0.21595_f64 * t7495;
    (t7779, t7782, t7785, t7788, t7791, t7800)
}
