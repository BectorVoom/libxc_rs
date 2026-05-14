//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 789/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk789<F: Float>(t2300: F, t3188: F, t3187: F, t2885: F, t820: F, t3192: F, t10103: F, t10106: F, t10108: F, t10111: F, t10115: F, t10118: F, t10120: F, t10126: F, t10128: F, t3206: F, t763: F) -> (F, F) {
    let t10130 = t3188 * t2300;
    let t10131 = t3187 * t10130;
    let t10133 = t2885 * t820;
    let t10134 = t3192 * t10133;
    let t10136 = -0.27357942622625364862e-5 * t10103 + 0.13223005600935593017e-4 * t10106 + 0.1252584660908875509e-2 * t10108 + 0.11742981196020707897e-5 * t10111 + 0.18788769913633132635e-4 * t10115 + 0.27357942622625364862e-5 * t10118 + 0.94840867758434598187e-4 * t10120 + 0.19798879235883268025e-5 * t10126 + 0.14615314396567373048e-4 * t10128 - 0.28183154870449698953e-3 * t10131 - 0.18788769913633132635e-4 * t10134;
    let t10137 = t763 * t3206;
    (t10136, t10137)
}
