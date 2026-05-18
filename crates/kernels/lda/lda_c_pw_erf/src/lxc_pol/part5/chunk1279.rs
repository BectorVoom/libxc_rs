//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1279/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1279<F: Float>(t1308: F, t571: F, t593: F, t7404: F, t1319: F, t21820: F, t2017: F, t21825: F, t2334: F, t811: F, t11914: F, t352: F, t3974: F) -> (F, F, F, F, F) {
    let t22915 = F::new(4.0) / F::new(45.0) * t571 * t1308 * t7404 * t593;
    let t22918 = F::new(8.0) / F::new(45.0) * t571 * t1319 * t21820;
    let t22921 = F::new(4.0) / F::new(27.0) * t571 * t2017 * t21825;
    let t22922 = t2334 * t811;
    let t22926 = F::new(64.0) / F::new(27.0) * t3974 * t11914 * t22922 * t352;
    (t22915, t22918, t22921, t22922, t22926)
}
