//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 816/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk816<F: Float>(t2: F, t37406: F, t37357: F, t3139: F, t466: F, t1781: F, t37362: F, t1775: F, t8308: F, t8314: F, t1791: F, t37352: F, t82: F, t37355: F, t1797: F, t8282: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38549 = t2 * t37406;
    let t38550 = t38549 * t37357;
    let t38554 = t3139 * t466;
    let t38556 = t1781 * t37362;
    let t38560 = t1775 * t8308;
    let t38562 = t8314 * t37357;
    let t38566 = t1791 * t37362;
    let t38570 = t37352 * t82;
    let t38571 = t2 * t37355;
    let t38572 = t38571 * t37357;
    let t38576 = t8282 * t1797;
    (t38550, t38554, t38556, t38560, t38562, t38566, t38570, t38572, t38576)
}
