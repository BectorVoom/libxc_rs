//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 559/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk559<F: Float>(t3188: F, t787: F, t3187: F, t283: F, t462: F, t2885: F, t315: F, t188: F, t291: F, t297: F, t2531: F, t799: F) -> (F, F, F, F, F, F, F, F) {
    let t3189 = t3188 * t787;
    let t3190 = t3187 * t3189;
    let t3192 = t462 * t283;
    let t3193 = t2885 * t315;
    let t3194 = t3192 * t3193;
    let t3196 = t188 * t291;
    let t3197 = t3196 * t297;
    let t3198 = t3197 * t2531;
    let t3199 = t799 * t3198;
    (t3189, t3190, t3192, t3193, t3194, t3197, t3198, t3199)
}
