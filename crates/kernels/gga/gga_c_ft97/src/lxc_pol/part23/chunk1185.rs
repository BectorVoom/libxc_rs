//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1185/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1185<F: Float>(t29424: F, t5: F, t24237: F, t30939: F, t30871: F, t30925: F, t107787: F, t24231: F, t27993: F, t28002: F, t28006: F, t28012: F, t28015: F, t28020: F, t28027: F, t28033: F, t28039: F, t3051: F, t30904: F, t6002: F, t6744: F, t684: F) -> (F, F) {
    let t115081 = t5 * t29424;
    let t121685 = t24237 * t30939;
    let t121694 = t24237 * t30871;
    let t121706 = t24237 * t30925;
    let t121708 = -t28015 * t27993 / 9.0 - t28015 * t28020 / 9.0 - t121685 / 27.0 - t28015 * t28002 / 9.0 - t28015 * t28006 / 9.0 + 2.0 / 9.0 * t6744 * t3051 * t28012 + t121694 / 54.0 + 2.0 / 9.0 * t28015 * t28027 + 2.0 / 9.0 * t28015 * t28033 - 2.0 / 27.0 * t28015 * t28039 + 2.0 / 9.0 * t6002 * t24231 * t30904 * t684 - 2.0 / 27.0 * t121706 - t107787;
    (t115081, t121708)
}
