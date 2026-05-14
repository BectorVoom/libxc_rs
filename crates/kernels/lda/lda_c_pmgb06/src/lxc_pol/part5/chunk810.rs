//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 810/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk810<F: Float>(t1710: F, t431: F, t2010: F, t446: F, t1981: F, t500: F, t1680: F, t1688: F, t1691: F, t4119: F, t1698: F, t1701: F, t208: F, t584: F, t586: F, t740: F) -> (F, F, F, F, F, F, F, F) {
    let t10318 = t431 * t1710;
    let t10321 = t2010 * t446;
    let t10335 = t1981 * t500;
    let t10350 = t1688 * t1680;
    let t10353 = t1691 * t4119;
    let t10356 = 4.0 / 9.0 * t1698 * t1680;
    let t10358 = 0.05402469135802469 * t1701 * t4119;
    let t10362 = 0.05402469135802469 * t584 * t586 * t740 * t208;
    (t10318, t10321, t10335, t10350, t10353, t10356, t10358, t10362)
}
