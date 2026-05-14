//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 400/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk400<F: Float>(t1063: F, t5: F, t1068: F, t2253: F, t179: F, t422: F, t71: F, t1576: F, t171: F, t11: F, t41: F, t1075: F, t1073: F, t2281: F, t184: F, t21: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3601 = t5 * t1063;
    let t3611 = t2253 * t1068;
    let t3613 = t422 * t179;
    let t3621 = t71 * t179;
    let t3626 = 1.0 / t171 / t1576;
    let t3627 = t11 * t3626;
    let t3628 = t41 * t3627;
    let t3633 = t2253 * t1075;
    let t3640 = t2281 * t1073;
    let t3664 = t184 * t21;
    (t3601, t3611, t3613, t3621, t3626, t3628, t3633, t3640, t3664)
}
