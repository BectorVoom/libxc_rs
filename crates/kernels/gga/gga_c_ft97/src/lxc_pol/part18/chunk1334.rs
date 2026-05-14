//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1334/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1334<F: Float>(t105717: F, t1369: F, t2112: F, t28: F, t105364: F, t446: F, t9073: F, t105526: F, t3281: F, t1017: F, t2120: F, t2185: F, t23657: F, t5900: F, t1636: F, t6681: F, t89: F) -> (F, F, F, F, F) {
    let t105720 = t1369 * t28 * t2112 * t105717;
    let t105722 = t446 * t9073 * t105364;
    let t105725 = t3281 * t9073 * t105526;
    let t105730 = t23657 * t2185 * t5900 * t1017 * t2120;
    let t105733 = t89 * t1636 * t6681;
    (t105720, t105722, t105725, t105730, t105733)
}
