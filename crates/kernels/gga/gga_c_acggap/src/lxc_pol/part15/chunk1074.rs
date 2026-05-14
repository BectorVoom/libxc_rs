//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1074/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1074<F: Float>(t30725: F, t32561: F, t34751: F, t37230: F, t37234: F, t39438: F, t39442: F, t39447: F, t39451: F, t39454: F, t39458: F, t39462: F, t39465: F, t39468: F, t39471: F, t39474: F, t39477: F) -> (F,) {
    let t41664 = -t37230 + 0.10482697429868050689e-2 * t39438 + 0.68598428988911579156e-2 * t34751 + 0.62896184579208304138e-3 * t39442 + t37234 + 0.31448092289604152069e-2 * t30725 + t32561 - 0.31448092289604152068e-2 * t39447 + 0.57165357490759649296e-3 * t39451 + 0.85748036236139473944e-3 * t39454 + 0.12579236915841660828e-2 * t39458 + 0.12579236915841660828e-2 * t39462 - t39465 / 8.0 + t39468 / 4.0 + t39471 / 12.0 + t39474 / 8.0 + t39477 / 24.0;
    (t41664,)
}
