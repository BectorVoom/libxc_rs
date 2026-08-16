//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 866/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk866(t3705: f64, t971: f64, t138: f64, t258: f64, t3834: f64, t3862: f64, t3818: f64, t3859: f64, t1058: f64, t696: f64, t965: f64, t3724: f64, t3758: f64, t963: f64) -> (f64, f64, f64, f64, f64) {
    let t8738 = t971 * t3705;
    let t8740 = t258 * t138;
    let t8743 = 0.13012297560362088_f64 * t8740 * t3834 * t3862;
    let t8746 = 1.9263893255070628_f64 * t8740 * t3818 * t3859;
    let t8755 = 21.053605041484726_f64 * t696 * t965 * t1058;
    let t8759 = 69.26343642272586_f64 * t696 * t963 * t3758 * t3724;
    (t8738, t8743, t8746, t8755, t8759)
}
