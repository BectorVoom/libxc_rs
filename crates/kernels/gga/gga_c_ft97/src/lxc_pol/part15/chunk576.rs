//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 576/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk576<F: Float>(t110: F, t1786: F, t463: F, t488: F, t100: F, t370: F, t8232: F, t981: F, t8326: F, t1780: F, t1637: F, t89: F, t973: F, t1771: F, t963: F, t2: F, t8275: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11468 = t1786 * t110;
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11578 = t89 * t1637 * t973;
    let t11669 = t1771 * t963;
    let t11690 = t8275 * t2;
    (t11468, t11472, t11490, t11550, t11552, t11556, t11578, t11669, t11690)
}
