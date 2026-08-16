//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 840/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk840(t123: f64, t295: f64, t297: f64, t315: f64, t317: f64, t4242: f64, t4249: f64, t4296: f64, t4301: f64, t4307: f64, t4322: f64, t4324: f64, t7425: f64, t77: f64, t7934: f64, t7937: f64, t8011: f64) -> f64 {
    let t8017 = -0.01197423401025461_f64 * t297 * t7934 + 6.0_f64 * t7937 * t77 + t4242 - t4249 + t8011 * t295 + 0.020267214298646783_f64 * t123 * t315 * t7425 * t317 - t4296 - t4301 + t4307 + t4322 - t4324;
    t8017
}
