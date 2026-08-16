//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 999/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk999(t11877: f64, t2872: f64, t493: f64, t1898: f64, t3213: f64, t161: f64, t3004: f64, t843: f64, t9350: f64, t11859: f64, t11861: f64, t11865: f64, t11867: f64, t11869: f64, t11872: f64, t11874: f64, t11876: f64) -> (f64, f64, f64, f64, f64) {
    let t11880 = 2.0_f64 / 15.0_f64 * t493 * t11877 * t2872;
    let t11881 = t3213 * t1898;
    let t11882 = 4.0_f64 / 135.0_f64 * t11881;
    let t11884 = t161 * t3004 * t843;
    let t11885 = 4.0_f64 / 405.0_f64 * t11884;
    let t11886 = 4.0_f64 / 135.0_f64 * t9350;
    let t11887 = t11859 - t11861 + t11865 - t11867 + t11869 - t11872 - t11874 - t11876 + t11880 + t11882 + t11885 + t11886;
    (t11880, t11882, t11885, t11886, t11887)
}
