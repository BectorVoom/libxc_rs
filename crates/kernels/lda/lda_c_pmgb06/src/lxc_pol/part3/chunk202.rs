//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 202/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk202<F: Float>(t504: F, t146: F, t164: F, t468: F, t163: F, t147: F) -> (F, F, F, F) {
    let t519 = F::new(0.035991666666666665) * t504;
    let t523 = F::new(0.006666666666666667) * t146 * t468 * t164;
    let t524 = F::new(1.0) / t163;
    let t525 = t147 * t524;
    (t519, t523, t524, t525)
}
