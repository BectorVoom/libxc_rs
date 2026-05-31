//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1045/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1045<F: Float>(t9352: F, t11882: F, t19485: F, t19488: F, t19493: F, t19497: F, t19498: F, t19499: F, t19504: F, t19507: F, t19508: F, t19509: F) -> (F, F) {
    let t19510 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t9352;
    let t19511 = -t19485 - t19488 + t19493 - t19497 + t19498 + t19499 + t19504 + t11882 - t19507 + t19508 + t19509 + t19510;
    (t19510, t19511)
}
