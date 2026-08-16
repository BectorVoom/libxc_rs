//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1224/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1224(t34508: f64, t34510: f64, t34516: f64, t34526: f64, t37126: f64, t37129: f64, t37132: f64, t37140: f64, t37142: f64, t39356: f64, t39358: f64, t39362: f64, t39364: f64, t39366: f64, t39368: f64, t39373: f64, t39377: f64) -> f64 {
    let t41610 = -0.20965394859736101379e-2_f64 * t39356 - 0.18868855373762491241e-2_f64 * t39358 + 0.42874018118069736972e-3_f64 * t39362 + 0.34299214494455789578e-2_f64 * t39364 + t37126 + 0.25724410870841842183e-2_f64 * t39366 + t37129 - 0.62896184579208304136e-2_f64 * t34508 + 0.26416397523267487737e-1_f64 * t34510 - 0.68598428988911579156e-2_f64 * t39368 - t37132 + 0.41930789719472202758e-2_f64 * t34516 + 0.16772315887788881104e-2_f64 * t34526 + 0.42874018118069736972e-2_f64 * t39373 - 0.94344276868812456207e-3_f64 * t39377 - t37140 + t37142;
    t41610
}
