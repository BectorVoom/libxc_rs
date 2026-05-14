//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 265/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk265<F: Float>(t519: F, t920: F, t356: F, t89: F, t528: F, t929: F, t126: F, t120: F, t534: F, t139: F) -> (F, F, F, F, F, F) {
    let t998 = t519 * t920;
    let t1000 = t89 * t356 * t998;
    let t1002 = t528 * t929;
    let t1005 = t929 * t126;
    let t1008 = 0.23410285231011484e0 * t1002 * t120 - 0.532971647967385935e-1 * t534 * t1005;
    let t1009 = t139 * t1008;
    (t998, t1000, t1002, t1005, t1008, t1009)
}
