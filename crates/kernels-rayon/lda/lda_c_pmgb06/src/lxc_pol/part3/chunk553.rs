//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 553/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk553(t161: f64, t2887: f64, t1499: f64, t531: f64, t513: f64, t1491: f64, t486: f64, t2854: f64, t2856: f64, t2859: f64, t2861: f64, t2863: f64, t2868: f64, t2870: f64, t2875: f64, t2879: f64, t2882: f64, t2884: f64) -> (f64, f64, f64, f64, f64) {
    let t2889 = t161 * t2887 / 10.0_f64;
    let t2891 = t1499 * t531 / 10.0_f64;
    let t2893 = t1499 * t513 / 10.0_f64;
    let t2895 = t486 * t1491 / 10.0_f64;
    let t2896 = t2854 - t2856 - t2859 - t2861 - t2863 + t2868 - t2870 + t2875 + t2879 - t2882 - t2884 - t2889 - t2891 + t2893 + t2895;
    (t2889, t2891, t2893, t2895, t2896)
}
