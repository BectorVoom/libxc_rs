//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1110/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1110<F: Float>(t36289: F, t36294: F, t37934: F, t37937: F, t37938: F, t37941: F, t37944: F, t37945: F, t40465: F, t40467: F, t40469: F, t40472: F, t40474: F, t40477: F, t40481: F, t40485: F, t40487: F) -> (F,) {
    let t42132 = -0.25158473831683321656e-2 * t40465 + 0.34299214494455789578e-2 * t40467 + 0.34299214494455789578e-2 * t40469 + t37934 + t37937 - t37938 - 0.75475421495049964965e-2 * t36289 + t37941 - 0.55907719625962937011e-2 * t36294 + t37944 + t37945 + 0.34299214494455789578e-2 * t40472 + 0.85748036236139473944e-3 * t40474 + t40477 / 8.0 + 0.18868855373762491242e-1 * t40481 - 0.75475421495049964966e-2 * t40485 + 0.42874018118069736972e-2 * t40487;
    (t42132,)
}
