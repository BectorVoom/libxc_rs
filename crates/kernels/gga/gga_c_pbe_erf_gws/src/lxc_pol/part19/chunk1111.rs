//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1111/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1111<F: Float>(t53896: F, t54014: F, t54052: F, t54072: F, t54087: F, t54102: F, t54113: F, t54117: F, t54128: F, t54135: F, t54152: F, t54166: F, t54198: F, t54236: F, t54238: F, t54257: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t55421 = 7.0 / 36.0 * t53896;
    let t55432 = 7.0 / 288.0 * t54014;
    let t55452 = 7.0 / 96.0 * t54052;
    let t55460 = 7.0 / 72.0 * t54072;
    let t55467 = 7.0 / 72.0 * t54087;
    let t55473 = 7.0 / 36.0 * t54102;
    let t55480 = 7.0 / 144.0 * t54113;
    let t55482 = 7.0 / 144.0 * t54117;
    let t55487 = 7.0 / 288.0 * t54128;
    let t55491 = 7.0 / 72.0 * t54135;
    let t55500 = 7.0 / 72.0 * t54152;
    let t55508 = 7.0 / 72.0 * t54166;
    let t55524 = 7.0 / 288.0 * t54198;
    let t55547 = 7.0 / 72.0 * t54236;
    let t55548 = 7.0 / 144.0 * t54238;
    let t55556 = 7.0 / 72.0 * t54257;
    (t55421, t55432, t55452, t55460, t55467, t55473, t55480, t55482, t55487, t55491, t55500, t55508, t55524, t55547, t55548, t55556)
}
