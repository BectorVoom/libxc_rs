//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1048/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1048<F: Float>(t12446: F, t12448: F, t12450: F, t12452: F, t12454: F, t12457: F, t12459: F, t12461: F, t12463: F, t12466: F, t12469: F, t2933: F, t5068: F, t852: F) -> (F, F) {
    let t12470 = t12446 - t12448 - t12450 + t12452 + t12454 - t12457 - t12459 - t12461 - t12463 - t12466 + t12469;
    let t12473 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t852 * t2933;
    (t12470, t12473)
}
