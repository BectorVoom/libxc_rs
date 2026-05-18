//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 852/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk852<F: Float>(t1681: F, t428: F, t1751: F, t397: F, t1711: F, t401: F, t1712: F, t398: F, t51: F, t6: F, t1609: F, t1610: F, t1613: F) -> (F, F, F, F, F) {
    let t37459 = t1681 * t428;
    let t37464 = t397 * t1751;
    let t37473 = t1711 * t401;
    let t37477 = t1712 * t6 * t51 * t398;
    let t37481 = t1613 * t1610 * t1609;
    (t37459, t37464, t37473, t37477, t37481)
}
