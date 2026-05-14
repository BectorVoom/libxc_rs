//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 788/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk788<F: Float>(t1971: F, t236: F, t495: F, t5605: F, t7453: F, t5888: F, t7230: F, t875: F, t3351: F, t498: F, t7231: F, t3352: F, t5149: F, t117: F, t29927: F, t2295: F) -> (F, F, F, F, F) {
    let t39620 = t7453 * t1971 * t236 * t5605 * t495;
    let t39625 = t7230 * t1971 * t875 * t5888 * t495;
    let t39630 = t3351 * t7231 * t875 * t5888 * t498;
    let t39635 = t3351 * t3352 * t875 * t5149;
    let t39649 = t29927 * t117;
    let t39650 = t39649 * t2295;
    (t39620, t39625, t39630, t39635, t39650)
}
