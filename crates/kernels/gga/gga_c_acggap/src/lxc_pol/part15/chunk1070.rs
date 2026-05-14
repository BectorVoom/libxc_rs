//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1070/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1070<F: Float>(t34508: F, t34510: F, t34516: F, t34526: F, t37126: F, t37129: F, t37132: F, t37140: F, t37142: F, t39356: F, t39358: F, t39362: F, t39364: F, t39366: F, t39368: F, t39373: F, t39377: F) -> (F,) {
    let t41610 = -0.20965394859736101379e-2 * t39356 - 0.18868855373762491241e-2 * t39358 + 0.42874018118069736972e-3 * t39362 + 0.34299214494455789578e-2 * t39364 + t37126 + 0.25724410870841842183e-2 * t39366 + t37129 - 0.62896184579208304136e-2 * t34508 + 0.26416397523267487737e-1 * t34510 - 0.68598428988911579156e-2 * t39368 - t37132 + 0.41930789719472202758e-2 * t34516 + 0.16772315887788881104e-2 * t34526 + 0.42874018118069736972e-2 * t39373 - 0.94344276868812456207e-3 * t39377 - t37140 + t37142;
    (t41610,)
}
