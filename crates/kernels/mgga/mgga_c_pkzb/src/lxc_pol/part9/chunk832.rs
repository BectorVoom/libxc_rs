//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 832/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk832<F: Float>(t2277: F, t361: F, t356: F, t2281: F, t6275: F, t2196: F, t828: F, t2199: F, t6143: F, t852: F, t2240: F, t369: F, t6121: F) -> (F, F, F, F, F, F, F, F) {
    let t6312 = 1.0 / t2277 / t361;
    let t6313 = t356 * t6312;
    let t6314 = t6275 * t2281;
    let t6317 = t828 * t2196;
    let t6319 = 6.0 * t6317 * t2199;
    let t6320 = t6143 * t852;
    let t6322 = 6.0 * t2240 * t6320;
    let t6323 = t369 * t6121;
    (t6312, t6313, t6314, t6317, t6319, t6320, t6322, t6323)
}
