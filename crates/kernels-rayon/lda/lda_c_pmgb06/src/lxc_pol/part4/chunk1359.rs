//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1359/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1359(t13973: f64, t187: f64, t7209: f64, t7179: f64, t13984: f64, t4754: f64, t824: f64, t1887: f64, t2043: f64, t1491: f64, t2563: f64, t161: f64, t489: f64, t6595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17858 = 2.0_f64 / 45.0_f64 * t13973;
    let t17859 = t7209 * t187;
    let t17861 = t7179 * t187;
    let t17863 = 8.0_f64 / 45.0_f64 * t13984;
    let t17869 = t4754 * t824 / 15.0_f64;
    let t17871 = 2.0_f64 / 15.0_f64 * t1887 * t2043;
    let t17873 = t2563 * t1491 / 30.0_f64;
    let t17875 = t161 * t489 * t6595;
    (t17858, t17859, t17861, t17863, t17869, t17871, t17873, t17875)
}
