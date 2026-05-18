//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1021/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1021<F: Float>(t3284: F, t7453: F, t11798: F, t190: F, t277: F, t11449: F, t11399: F, t286: F) -> (F, F, F, F, F) {
    let t11799 = t3284 * t7453;
    let t11800 = t11798 * t11799;
    let t11802 = t277 * t190;
    let t11803 = t11802 * t11449;
    let t11804 = t11399 * t286;
    (t11799, t11800, t11802, t11803, t11804)
}
