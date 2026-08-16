//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1344/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1344(t13916: f64, t13917: f64, t22332: f64, t22334: f64, t22338: f64, t22341: f64, t22343: f64, t22345: f64, t22350: f64, t22352: f64, t22354: f64, t22358: f64, t22361: f64) -> f64 {
    let t23304 = t22332 - t22334 + t22338 + t22341 + t22343 + t22345 + t22350 + t22352 + t22354 + t22358 + t22361 + t13916 + 0.6492624817418906_f64 * t13917;
    t23304
}
