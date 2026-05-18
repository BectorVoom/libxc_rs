//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1180/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1180<F: Float>(t2001: F, t6121: F, t1165: F, t2068: F, t26214: F, t7351: F, t7433: F, t9593: F, t5612: F, t7822: F, t5743: F, t8511: F) -> (F, F, F, F, F) {
    let t40264 = t2001 * t6121;
    let t40268 = t2068 * t1165 * t7351 * t26214;
    let t40270 = t7433 * t9593;
    let t40272 = t7822 * t5612;
    let t40274 = t8511 * t5743;
    (t40264, t40268, t40270, t40272, t40274)
}
