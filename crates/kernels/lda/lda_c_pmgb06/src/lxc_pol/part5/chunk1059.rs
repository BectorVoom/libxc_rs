//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1059/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1059<F: Float>(t16104: F, t12038: F, t19679: F, t19680: F, t19681: F, t19682: F, t19683: F, t19685: F, t19687: F, t19689: F, t19693: F, t16106: F) -> (F, F, F) {
    let t19694 = F::new(4.0) / F::new(135.0) * t16104;
    let t19695 = t19679 + t19680 - t19681 + t19682 + t19683 + t19685 - t19687 - t19689 - t19693 - t12038 - t19694;
    let t19696 = F::new(4.0) / F::new(135.0) * t16106;
    (t19694, t19695, t19696)
}
