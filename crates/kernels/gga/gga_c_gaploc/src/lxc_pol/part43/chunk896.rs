//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 896/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk896<F: Float>(t40744: F, t43095: F, t43099: F, t43101: F, t43106: F, t43111: F, t43115: F, t43119: F, t43122: F, t43125: F, t43131: F, t43143: F, t43146: F, t43148: F, t43152: F, t43156: F, t43157: F, t47661: F) -> (F,) {
    let t51016 = -0.17090058289204942853e-2 * t43095 + t43099 + t43101 - t47661 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + t43131 - t43143 + t43146 + 0.12817543716903707139e-2 * t40744 - t43148 - t43152 + t43156 + t43157;
    (t51016,)
}
