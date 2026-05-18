//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 850/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk850<F: Float>(t1608: F, t1613: F, t373: F, t408: F, t1696: F, t428: F, t8002: F, t1609: F, t7905: F, t1597: F, t7899: F, t63: F) -> (F, F, F, F, F) {
    let t37443 = t1608 * t408 * t1613 * t373;
    let t37445 = t8002 * t1696 * t428;
    let t37452 = t1609 * t7905;
    let t37453 = t7899 * t1597;
    let t37454 = t37453 * t63;
    (t37443, t37445, t37452, t37453, t37454)
}
