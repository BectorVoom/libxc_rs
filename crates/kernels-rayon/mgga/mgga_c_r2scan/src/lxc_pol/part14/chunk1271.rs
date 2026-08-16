//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1271/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1271(t1149: f64, t354: f64, t374: f64, t41811: f64, t41814: f64, t41818: f64, t41821: f64, t41824: f64, t42133: f64, t42136: f64, t42138: f64, t42140: f64, t42143: f64, t42146: f64, t42148: f64, t42161: f64, t42166: f64, t42171: f64, t42175: f64, t42182: f64, t42192: f64, t42199: f64, t42204: f64, t42213: f64, t42221: f64, t42231: f64, t42236: f64, t42239: f64, t42244: f64, t42249: f64, t42275: f64, t42298: f64, t42327: f64, t42350: f64, t8505: f64) -> f64 {
    let t42356 = t41811 - t41814 - t41818 - t41821 - t41824 + t42133 * t374 + t42136 - t42138 + t42140 - t42143 + t42146 - t42148 + t8505 * t1149 + t354 * (t42161 + t42166 + t42171 + t42175 + t42182 + t42192 + t42199 + t42204 + t42213 + t42221 + t42231 + t42249 + t42275 + t42298 + t42327 + t42350) + t42236 + t42239 - t42244;
    t42356
}
