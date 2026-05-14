//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 632/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk632<F: Float>(t1730: F, t1735: F, t5124: F, t5128: F, t5132: F, t5136: F, t5140: F, t5144: F, t5148: F, t5151: F, t5154: F, t5158: F, t5160: F, t5166: F, t5168: F, t5170: F, t5173: F, t5181: F, t5183: F) -> (F, F) {
    let t5185 = 4.0 / 5.0 * t1730 * t1735;
    let t5186 = -t5124 + t5128 - t5132 + t5136 - t5140 + t5144 - t5148 - t5151 + t5154 + t5158 - t5160 + t5166 + t5168 + t5170 + t5173 - t5181 + t5183 + t5185;
    (t5185, t5186)
}
