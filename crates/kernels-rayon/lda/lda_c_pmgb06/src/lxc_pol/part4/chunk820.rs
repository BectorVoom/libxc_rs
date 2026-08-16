//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 820/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk820(t123: f64, t2164: f64, t317: f64, t740: f64, t117: f64, t2360: f64, t315: f64, t1179: f64, t794: f64, t419: f64, t421: f64, t1798: f64, t409: f64) -> (f64, f64, f64, f64, f64) {
    let t5601 = 0.10809180959278285_f64 * t123 * t740 * t2164 * t317;
    let t5610 = 0.017961351015381915_f64 * t123 * t315 * t2360 * t117;
    let t5613 = t1179 * t794;
    let t5615 = t5613 * t419 * t421;
    let t5617 = t409 * t1798;
    (t5601, t5610, t5613, t5615, t5617)
}
