//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 902/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk902<F: Float>(t3051: F, t471: F, t458: F, t8272: F, t2: F, t32075: F, t1771: F, t1806: F, t1802: F, t11176: F, t94: F, t432: F, t8376: F) -> (F, F, F, F, F, F, F) {
    let t38504 = t3051 * t471;
    let t38506 = t458 * t8272;
    let t38508 = t32075 * t2;
    let t38513 = t1771 * t1806;
    let t38519 = t1771 * t1802;
    let t38525 = F::new(280.0) / F::new(81.0) * t11176 * t94;
    let t38526 = t8376 * t432;
    (t38504, t38506, t38508, t38513, t38519, t38525, t38526)
}
