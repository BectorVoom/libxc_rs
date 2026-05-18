//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1147/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1147<F: Float>(t432: F, t4975: F, t1586: F, t161: F, t166: F, t4801: F, t13675: F, t13676: F, t13677: F, t13678: F, t13682: F, t13684: F, t13687: F, t13689: F, t13693: F, t13697: F) -> (F, F, F) {
    let t13699 = t432 * t4975 / F::new(5.0);
    let t13703 = t161 * t166 * t4801 * t1586 / F::new(10.0);
    let t13704 = -t13675 - t13676 - t13677 + t13678 - t13682 - t13684 - t13687 - t13689 - t13693 - t13697 - t13699 - t13703;
    (t13699, t13703, t13704)
}
