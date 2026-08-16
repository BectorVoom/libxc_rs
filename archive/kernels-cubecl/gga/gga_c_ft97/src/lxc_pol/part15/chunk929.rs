//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 929/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk929<F: Float>(t19233: F, t287: F, t19106: F, t4092: F, t1771: F, t5360: F, t5356: F, t5352: F, t8282: F, t5346: F, t5349: F, t1636: F, t5226: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t70671 = t19233 * t287;
    let t70779 = t4092 * t19106;
    let t70799 = t1771 * t5360;
    let t70801 = t1771 * t5356;
    let t70826 = t8282 * t5352;
    let t70935 = t8282 * t5346;
    let t70999 = t8282 * t5349;
    let t71238 = t89 * t1636 * t5226;
    (t70671, t70779, t70799, t70801, t70826, t70935, t70999, t71238)
}
