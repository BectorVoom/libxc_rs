//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 389/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk389<F: Float>(t79: F, t409: F, t4491: F, t64: F, t1599: F, t1624: F, t372: F, t4442: F, t4446: F, t4450: F, t4468: F, t4471: F, t4476: F) -> (F, F) {
    let t80 = 0.1e-59 < t79;
    let t4492 = t409 * t4491;
    let t4493 = t64 * t4492;
    let t4495 = piecewise3(t80, 0.67598802253579164263e-4 * t4442 * t1599 + 0.23254900946437792e-1 * t1624 * t4446 + 0.23254900946437792e-2 * t372 * t4450 - 0.11627450473218896e-1 * t372 * t4468 + 0.19365723406274399941e-3 * t372 * t4471 + 2.0 * t4476 - t4493, 0.0);
    (t4493, t4495)
}
