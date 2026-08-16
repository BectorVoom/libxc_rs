//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 620/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk620(t2801: f64, t1338: f64, t415: f64, t1139: f64, t118: f64, t718: f64, t1166: f64, t81: f64, t1329: f64, t1186: f64, t1334: f64, t421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2802 = 24.0_f64 * t2801;
    let t2807 = t1338 * t415;
    let t2809 = t1139 * t118;
    let t2812 = 0.1890324433388467_f64 * t718 * t415;
    let t2813 = t81 * t1166;
    let t2814 = t2813 * t118;
    let t2816 = t1329 * t415;
    let t2820 = 0.01975389032890948_f64 * t1334 * t1186 * t421;
    (t2802, t2807, t2809, t2812, t2813, t2814, t2816, t2820)
}
