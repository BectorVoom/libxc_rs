//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 651/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk651(t2262: f64, t707: f64, t2266: f64, t1773: f64, t909: f64, t123: f64, t2164: f64, t317: f64, t740: f64, t117: f64, t2360: f64, t315: f64) -> (f64, f64, f64, f64, f64) {
    let t5590 = 0.039914113367515366_f64 * t707 * t2262;
    let t5591 = t707 * t2266;
    let t5593 = t1773 * t909;
    let t5601 = 0.10809180959278285_f64 * t123 * t740 * t2164 * t317;
    let t5610 = 0.017961351015381915_f64 * t123 * t315 * t2360 * t117;
    (t5590, t5591, t5593, t5601, t5610)
}
