//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 547/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk547(t154: f64, t2851: f64, t132: f64, t1548: f64, t432: f64, t1547: f64, t459: f64, t1592: f64, t442: f64, t1600: f64, t496: f64, t1179: f64, t139: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2852 = t2851 * t154;
    let t2854 = 4.0_f64 / 405.0_f64 * t132 * t2852;
    let t2855 = t432 * t1548;
    let t2857 = t1547 * t459;
    let t2858 = t132 * t2857;
    let t2864 = t442 * t1592;
    let t2871 = t496 * t1600;
    let t2897 = t1179 * t139;
    (t2852, t2854, t2855, t2857, t2858, t2864, t2871, t2897)
}
