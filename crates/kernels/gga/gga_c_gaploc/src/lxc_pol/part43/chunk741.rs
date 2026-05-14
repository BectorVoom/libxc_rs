//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 741/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk741<F: Float>(t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F, t12782: F, t64: F, t10205: F, t871: F, t40353: F, t9078: F, t986: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t42111 = (21.0 / 512.0 * t39624 + 357.0 / 16384.0 * t39626 - 189.0 / 262144.0 * t39632 + 189.0 / 0.16777216e8 * t39637 - 63.0 / 0.16777216e8 * t39642 + 63.0 / 262144.0 * t39646 - 119.0 / 16384.0 * t39648 - 7.0 / 512.0 * t39650) * t471;
    let t42113 = 4.0 / 3.0 * t12782 * t64;
    let t42114 = t10205 * t871;
    let t42117 = 7.0 / 512.0 * t39624;
    let t42118 = 63.0 / 16384.0 * t39626;
    let t42119 = 63.0 / 1048576.0 * t39632;
    let t42120 = 21.0 / 1048576.0 * t39646;
    let t42121 = 21.0 / 16384.0 * t39648;
    let t42122 = 7.0 / 1536.0 * t39650;
    let t42144 = 0.11502877786176224903e1 * t40353;
    let t42148 = t9078 * t986;
    (t42111, t42113, t42114, t42117, t42118, t42119, t42120, t42121, t42122, t42144, t42148)
}
