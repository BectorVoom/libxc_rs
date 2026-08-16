//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1031/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1031(t12252: f64, t132: f64, t137: f64, t1594: f64, t12227: f64, t12230: f64, t12233: f64, t12235: f64, t12237: f64, t12240: f64, t12242: f64, t12244: f64, t12246: f64, t12249: f64, t12251: f64) -> (f64, f64) {
    let t12256 = t132 * t137 * t12252 * t1594 / 5.0_f64;
    let t12257 = -2.0_f64 / 9.0_f64 * t12227 + t12230 + t12233 + t12235 - t12237 + t12240 + t12242 - t12244 + t12246 - t12249 + t12251 + t12256;
    (t12256, t12257)
}
