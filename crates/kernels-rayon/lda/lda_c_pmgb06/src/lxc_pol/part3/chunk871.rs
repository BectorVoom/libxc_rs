//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 871/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk871(t3903: f64, t643: f64, t3934: f64, t638: f64, t1058: f64, t696: f64, t965: f64, t3724: f64, t3758: f64, t963: f64, t3729: f64, t971: f64) -> (f64, f64, f64, f64, f64) {
    let t8749 = t643 * t3903;
    let t8751 = t638 * t3934;
    let t8755 = 21.053605041484726_f64 * t696 * t965 * t1058;
    let t8759 = 69.26343642272586_f64 * t696 * t963 * t3758 * t3724;
    let t8760 = t971 * t3729;
    (t8749, t8751, t8755, t8759, t8760)
}
