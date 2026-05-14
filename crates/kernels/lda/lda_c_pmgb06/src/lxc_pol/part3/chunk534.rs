//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 534/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk534<F: Float>(t2885: F, t529: F, t166: F, t161: F, t1499: F, t531: F, t513: F, t1491: F, t486: F, t2854: F, t2856: F, t2859: F, t2861: F, t2863: F, t2868: F, t2870: F, t2875: F, t2879: F, t2882: F, t2884: F) -> (F, F, F, F, F, F, F) {
    let t2886 = t2885 * t529;
    let t2887 = t166 * t2886;
    let t2889 = t161 * t2887 / 10.0;
    let t2891 = t1499 * t531 / 10.0;
    let t2893 = t1499 * t513 / 10.0;
    let t2895 = t486 * t1491 / 10.0;
    let t2896 = t2854 - t2856 - t2859 - t2861 - t2863 + t2868 - t2870 + t2875 + t2879 - t2882 - t2884 - t2889 - t2891 + t2893 + t2895;
    (t2886, t2887, t2889, t2891, t2893, t2895, t2896)
}
