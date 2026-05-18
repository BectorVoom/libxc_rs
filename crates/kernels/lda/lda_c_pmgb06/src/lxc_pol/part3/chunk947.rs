//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 947/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk947<F: Float>(t8614: F, t8589: F, t8594: F, t8598: F, t8603: F, t8605: F, t8610: F, t8612: F, t8616: F, t8621: F, t8626: F, t8629: F, t8633: F, t8637: F, t8640: F, t8644: F) -> F {
    let t11083 = F::new(240.0) * t8614;
    let t11085 = t8589 - t8594 - t8598 + t8603 + t8605 + t8610 - t8612 + t11083 + F::new(60.0) * t8616 + t8621 - t8626 - t8629 - t8633 - t8637 + t8640 + t8644;
    t11085
}
