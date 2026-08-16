//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1199/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1199(t12246: f64, t12249: f64, t12251: f64, t12256: f64, t12260: f64, t12265: f64, t12267: f64, t12269: f64, t12271: f64, t12273: f64, t12275: f64, t10687: f64, t10690: f64, t12277: f64, t12279: f64, t12282: f64, t12300: f64, t12302: f64, t12315: f64, t12415: f64, t12417: f64, t12435: f64, t12437: f64) -> (f64, f64) {
    let t14366 = t12246 - t12249 + t12251 + t12256 - t12260 - t12265 - t12267 - t12269 - t12271 - t12273 + t12275;
    let t14367 = t12277 + t12279 + t12282 + t12300 + t12302 - t10687 + t10690 + t12315 + t12415 + t12417 + t12435 + t12437;
    (t14366, t14367)
}
