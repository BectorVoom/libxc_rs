//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 981/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk981<F: Float>(t1386: F, t3290: F, t5077: F, t822: F, t10137: F, t10156: F, t10134: F, t13295: F, t13299: F, t13303: F, t13307: F, t13311: F, t13313: F, t13315: F, t13318: F, t10158: F) -> (F, F, F, F, F) {
    let t13322 = 4.0 / 15.0 * t5077 * t3290 * t822 * t1386;
    let t13323 = 2.0 / 45.0 * t10137;
    let t13324 = 4.0 / 45.0 * t10156;
    let t13325 = -t13295 + t13299 - t10134 + t13303 - t13307 - t13311 + t13313 + t13315 - t13318 + t13322 - t13323 - t13324;
    let t13327 = 2.0 / 27.0 * t10158;
    (t13322, t13323, t13324, t13325, t13327)
}
