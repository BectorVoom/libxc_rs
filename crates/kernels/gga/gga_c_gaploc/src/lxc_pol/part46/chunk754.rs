//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 754/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk754<F: Float>(t12782: F, t64: F, t10205: F, t871: F, t2748: F, t3113: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t42111: F, t493: F, t1: F, t1415: F, t2413: F, t31730: F) -> (F, F, F) {
    let t42113 = 4.0 / 3.0 * t12782 * t64;
    let t42114 = t10205 * t871;
    let t42115 = t2748 * t3113;
    let t42117 = 7.0 / 512.0 * t39624;
    let t42118 = 63.0 / 16384.0 * t39626;
    let t42119 = 63.0 / 1048576.0 * t39632;
    let t42120 = 21.0 / 1048576.0 * t39646;
    let t42121 = 21.0 / 16384.0 * t39648;
    let t42122 = 7.0 / 1536.0 * t39650;
    let t42123 = t42111 - t42113 + t42114 - t42115 / 2.0 + t42117 + t42118 - t42119 + t42120 - t42121 - t42122;
    let t42130 = t493 * t42123;
    let t42138 = t1415 * t31730 * t1 * t2413;
    (t42123, t42130, t42138)
}
