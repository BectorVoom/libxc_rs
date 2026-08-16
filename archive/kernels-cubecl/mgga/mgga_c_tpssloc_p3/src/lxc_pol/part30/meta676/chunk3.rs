//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2110/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110<F: Float>(t5445: F, t641: F, t72: F, t19445: F, t79: F, t2240: F, t27948: F, t33: F, t55921: F, t6489: F, t19299: F, t608: F) -> (F, F, F, F, F) {
    let t96517 = t72 * t641 * t5445;
    let t96521 = t72 * t79 * t19445;
    let t96529 = t2240 * t33 * t27948;
    let t96532 = t55921 * t6489;
    let t96535 = t19299 * t608;
    (t96517, t96521, t96529, t96532, t96535)
}
