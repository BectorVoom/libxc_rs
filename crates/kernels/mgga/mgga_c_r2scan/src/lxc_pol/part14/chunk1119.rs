//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1119/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1119<F: Float>(t1149: F, t354: F, t374: F, t41811: F, t41814: F, t41818: F, t41821: F, t41824: F, t42133: F, t42136: F, t42138: F, t42140: F, t42143: F, t42146: F, t42148: F, t42161: F, t42166: F, t42171: F, t42175: F, t42182: F, t42192: F, t42199: F, t42204: F, t42213: F, t42221: F, t42231: F, t42236: F, t42239: F, t42244: F, t42249: F, t42275: F, t42298: F, t42327: F, t42350: F, t8505: F) -> (F,) {
    let t42356 = t41811 - t41814 - t41818 - t41821 - t41824 + t42133 * t374 + t42136 - t42138 + t42140 - t42143 + t42146 - t42148 + t8505 * t1149 + t354 * (t42161 + t42166 + t42171 + t42175 + t42182 + t42192 + t42199 + t42204 + t42213 + t42221 + t42231 + t42249 + t42275 + t42298 + t42327 + t42350) + t42236 + t42239 - t42244;
    (t42356,)
}
