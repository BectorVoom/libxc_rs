//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 883/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk883<F: Float>(t10101: F, t10146: F, t10239: F, t10280: F, t158: F, t3909: F, t6546: F, t951: F, t3254: F, t3278: F, t2428: F, t3928: F, t3874: F, t410: F, t6514: F, t10121: F, t7832: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10282 = t10101 + t10146 + t10239 + t10280;
    let t10283 = t10282 * t158;
    let t10296 = t6546 * t3909;
    let t10297 = t10296 * t951;
    let t10300 = t3254 * t3278;
    let t10305 = t2428 * t3928;
    let t10306 = t10305 * t951;
    let t10309 = t410 * t3874;
    let t10310 = t6514 * t10309;
    let t10311 = t7832 * t10121;
    (t10282, t10283, t10296, t10297, t10300, t10306, t10309, t10310, t10311)
}
