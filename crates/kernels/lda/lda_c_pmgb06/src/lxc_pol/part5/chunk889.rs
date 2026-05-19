//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 889/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk889<F: Float>(t1680: F, t1698: F, t1701: F, t4119: F, t208: F, t584: F, t586: F, t740: F, t3260: F, t464: F, t3031: F, t442: F) -> (F, F, F, F, F) {
    let t10356 = F::new(4.0) / F::new(9.0) * t1698 * t1680;
    let t10358 = F::cast_from(0.05402469135802469_f64) * t1701 * t4119;
    let t10362 = F::cast_from(0.05402469135802469_f64) * t584 * t586 * t740 * t208;
    let t10431 = t3260 * t464;
    let t10439 = t442 * t3031;
    (t10356, t10358, t10362, t10431, t10439)
}
