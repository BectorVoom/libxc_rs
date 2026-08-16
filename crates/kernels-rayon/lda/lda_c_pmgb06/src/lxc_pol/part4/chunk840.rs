//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 840/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk840(t5770: f64, t5772: f64, t2233: f64, t342: f64, t1227: f64, t780: f64, t348: f64, t776: f64, t110: f64, t2217: f64, t360: f64, t2186: f64, t947: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5774 = 5.87616_f64 * t5770 * t5772;
    let t5775 = t2233 * t342;
    let t5779 = t780 * t1227;
    let t5783 = t348 * t776;
    let t5785 = 1.9486833333333333_f64 * t5783 * t5772;
    let t5787 = t360 * t110 * t2217;
    let t5788 = t2186 * t947;
    (t5774, t5775, t5779, t5783, t5785, t5787, t5788)
}
