//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1000/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1000<F: Float>(t13686: F, t4754: F, t479: F, t132: F, t137: F, t2064: F, t3058: F, t161: F, t166: F, t2093: F, t3382: F, t432: F, t4975: F, t1586: F, t4801: F, t13675: F, t13676: F, t13677: F, t13678: F, t13682: F, t13684: F) -> (F, F, F, F, F, F, F) {
    let t13687 = 2.0 / 15.0 * t13686;
    let t13689 = t4754 * t479 / 10.0;
    let t13693 = t132 * t137 * t3058 * t2064 / 10.0;
    let t13697 = t161 * t166 * t2093 * t3382 / 30.0;
    let t13699 = t432 * t4975 / 5.0;
    let t13703 = t161 * t166 * t4801 * t1586 / 10.0;
    let t13704 = -t13675 - t13676 - t13677 + t13678 - t13682 - t13684 - t13687 - t13689 - t13693 - t13697 - t13699 - t13703;
    (t13687, t13689, t13693, t13697, t13699, t13703, t13704)
}
