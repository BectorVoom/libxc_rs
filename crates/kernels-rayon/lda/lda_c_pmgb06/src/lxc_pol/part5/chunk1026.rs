//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1026/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1026(t19252: f64, t19254: f64, t19256: f64, t19258: f64, t19260: f64, t19263: f64, t19265: f64, t19268: f64, t19271: f64, t19274: f64, t19276: f64, t15481: f64) -> (f64, f64) {
    let t19277 = t19252 + t19254 + t19256 + t19258 + t19260 + t19263 + t19265 + t19268 + t19271 - t19274 + t19276;
    let t19278 = 2.0_f64 / 15.0_f64 * t15481;
    (t19277, t19278)
}
