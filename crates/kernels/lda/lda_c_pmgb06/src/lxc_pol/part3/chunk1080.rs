//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1080/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1080<F: Float>(t12831: F, t9760: F, t9762: F, t9765: F, t9771: F, t12823: F, t12824: F, t12826: F, t12827: F, t12829: F, t9759: F, t9770: F) -> (F, F, F, F, F, F) {
    let t12832 = t12831 / F::new(45.0);
    let t12833 = t9760 / F::new(15.0);
    let t12834 = F::new(4.0) / F::new(135.0) * t9762;
    let t12835 = F::new(4.0) / F::new(135.0) * t9765;
    let t12836 = t9771 / F::new(15.0);
    let t12837 = -t12823 - t12824 - t12826 + t12827 + t9759 - t12829 - t12832 + t12833 + t12834 + t12835 - t9770 + t12836;
    (t12832, t12833, t12834, t12835, t12836, t12837)
}
