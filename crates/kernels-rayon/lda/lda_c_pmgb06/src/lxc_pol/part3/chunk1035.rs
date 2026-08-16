//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1035/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1035(t513: f64, t5432: f64, t12260: f64, t12265: f64, t12267: f64, t12269: f64, t12271: f64, t12273: f64, t12275: f64, t12277: f64, t12279: f64, t12282: f64, t12300: f64) -> (f64, f64) {
    let t12302 = t5432 * t513 / 10.0_f64;
    let t12303 = -t12260 - t12265 - t12267 - t12269 - t12271 - t12273 + t12275 + t12277 + t12279 + t12282 + t12300 + t12302;
    (t12302, t12303)
}
