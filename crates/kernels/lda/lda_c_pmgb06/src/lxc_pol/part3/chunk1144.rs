//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1144/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1144<F: Float>(t132: F, t13554: F, t13591: F, t13623: F, t13662: F, t137: F, t465: F, t13508: F, t13510: F, t13512: F, t13514: F, t13516: F, t13519: F, t13521: F, t13525: F, t13527: F, t13529: F, t13530: F) -> (F, F) {
    let t13668 = t132 * t137 * t465 * (t13554 + t13591 + t13623 + t13662) / F::new(30.0);
    let t13669 = t13508 + t13510 + t13512 + t13514 + t13516 + t13519 + t13521 + t13525 - t13527 - t13529 - t13530 - t13668;
    (t13668, t13669)
}
