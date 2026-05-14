//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 490/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk490<F: Float>(t3106: F, t467: F, t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t2287: F, t871: F, t3109: F, t471: F, t64: F) -> (F, F, F, F) {
    let t9115 = t3106 * t467;
    let t9117 = -21.0 / 512.0 * t9097 + 147.0 / 16384.0 * t9100 - 63.0 / 1048576.0 * t9108 + 21.0 / 1048576.0 * t9111 - 49.0 / 16384.0 * t9113 + 7.0 / 512.0 * t9115;
    let t9121 = t2287 * t871;
    let t9127 = t9117 * t471 - 4.0 / 3.0 * t3109 * t64 + t9121 / 2.0 - 7.0 / 512.0 * t9097 + 21.0 / 16384.0 * t9100 - 7.0 / 16384.0 * t9113 + 7.0 / 1536.0 * t9115;
    (t9115, t9117, t9121, t9127)
}
