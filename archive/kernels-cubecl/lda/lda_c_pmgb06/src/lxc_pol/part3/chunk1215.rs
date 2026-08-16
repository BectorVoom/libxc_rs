//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1215/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1215<F: Float>(t13519: F, t13521: F, t13525: F, t13527: F, t13529: F, t13530: F, t13668: F, t13675: F, t13676: F, t13677: F, t13678: F, t13682: F, t13684: F, t13687: F, t13689: F, t13693: F, t13697: F, t13699: F, t13703: F, t13707: F, t13709: F, t13711: F, t13714: F) -> (F, F) {
    let t14436 = t13519 + t13521 + t13525 - t13527 - t13529 - t13530 - t13668 - t13675 - t13676 - t13677 + t13678;
    let t14437 = -t13682 - t13684 - t13687 - t13689 - t13693 - t13697 - t13699 - t13703 + t13707 - t13709 - t13711 - t13714;
    (t14436, t14437)
}
