//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 606/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk606<F: Float>(t103: F, t3336: F, t3338: F, t3341: F, t3344: F, t3347: F, t3350: F, t3352: F, t3354: F, t3359: F, t3362: F, t3368: F) -> F {
    let t3369 = -F::new(0.02666666666666667) * t3336 + F::new(0.013333333333333334) * t103 * t3338 - F::new(0.006666666666666667) * t103 * t3341 - F::new(0.04) * t103 * t3344 + F::new(0.04) * t103 * t3347 - F::new(0.022222222222222223) * t3350 + F::new(0.013333333333333334) * t3352 + F::new(0.0044444444444444444) * t3354 - F::new(0.002962962962962963) * t103 * t3359 - F::new(0.006666666666666667) * t103 * t3362 - t3368;
    t3369
}
