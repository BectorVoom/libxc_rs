//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 840/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk840<F: Float>(t123: F, t295: F, t297: F, t315: F, t317: F, t4242: F, t4249: F, t4296: F, t4301: F, t4307: F, t4322: F, t4324: F, t7425: F, t77: F, t7934: F, t7937: F, t8011: F) -> F {
    let t8017 = -F::cast_from(0.01197423401025461_f64) * t297 * t7934 + F::cast_from(6.0_f64) * t7937 * t77 + t4242 - t4249 + t8011 * t295 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t7425 * t317 - t4296 - t4301 + t4307 + t4322 - t4324;
    t8017
}
