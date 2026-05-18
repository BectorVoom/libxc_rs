//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 979/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk979<F: Float>(t1771: F, t2118: F, t3051: F, t588: F, t458: F, t9186: F, t2114: F, t2: F, t32905: F, t9238: F, t1775: F, t9196: F) -> (F, F, F, F, F, F, F) {
    let t40368 = t1771 * t2118;
    let t40370 = t3051 * t588;
    let t40375 = t458 * t9186;
    let t40377 = t1771 * t2114;
    let t40379 = t32905 * t2;
    let t40384 = t458 * t9238;
    let t40392 = t1775 * t9196;
    (t40368, t40370, t40375, t40377, t40379, t40384, t40392)
}
