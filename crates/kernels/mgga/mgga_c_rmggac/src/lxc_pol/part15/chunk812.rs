//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 812/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk812<F: Float>(t39277: F, t9206: F, t10014: F, t36662: F, t1907: F, t236: F, t321: F, t3352: F, t7230: F, t17859: F, t9106: F, t9111: F, t2283: F, t38472: F, t2286: F, t38638: F) -> (F, F, F, F, F, F, F) {
    let t45316 = t39277 * t9206;
    let t45318 = t36662 * t10014;
    let t45323 = t7230 * t3352 * t236 * t1907 * t321;
    let t45325 = t17859 * t9106;
    let t45327 = t17859 * t9111;
    let t45329 = t38472 * t2283;
    let t45331 = t38638 * t2286;
    (t45316, t45318, t45323, t45325, t45327, t45329, t45331)
}
