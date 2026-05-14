//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1206/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1206<F: Float>(t224: F, t36263: F, t36300: F, t36458: F, t37309: F, t2036: F, t36095: F, t36098: F, t36100: F, t36103: F, t36105: F, t36108: F, t36109: F, t36111: F, t36113: F, t36116: F, t36119: F, t36122: F, t36124: F, t36270: F, t36271: F, t36275: F, t36283: F, t3848: F) -> (F,) {
    let t37312 = t224 * (t36263 + t36300 + t36458 + t37309);
    let t37313 = t2036 * t3848 - t36095 - t36098 - t36100 - t36103 + t36105 - t36108 - t36109 + t36111 - t36113 - t36116 + t36119 + t36122 + 2.0 * t36124 - t36270 - t36271 - t36275 + t36283 + t37312;
    (t37313,)
}
