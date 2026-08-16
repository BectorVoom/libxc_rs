//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1040/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1040(t12150: f64, t1525: f64, t1830: f64, t12156: f64, t36: f64, t9188: f64, t12161: f64, t3090: f64, t1863: f64, t3115: f64, t453: f64, t12165: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12374 = t1830 * t1525 * t12150;
    let t12377 = t36 * t9188 * t12156;
    let t12380 = t1830 * t3090 * t12161;
    let t12382 = t1863 * t3115;
    let t12384 = t36 * t453 * t12382;
    let t12387 = t1830 * t453 * t12165;
    (t12374, t12377, t12380, t12382, t12384, t12387)
}
