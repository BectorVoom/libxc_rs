//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 812/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk812<F: Float>(t1808: F, t1766: F, t91: F, t1586: F, t22: F, t36452: F, t37991: F, t96: F, t1767: F, t1775: F, t8324: F, t1554: F, t2: F, t355: F, t7241: F, t369: F, t7760: F) -> (F, F, F, F, F, F, F, F) {
    let t38447 = t1808 * t1808;
    let t38449 = t91 * t1766 * t38447;
    let t38456 = 1.0 / t96 / t37991 / t22 / t1586 / t36452 / 96.0;
    let t38457 = t1767 * t1767;
    let t38459 = t91 * t38456 * t38457;
    let t38461 = t1775 * t8324;
    let t38463 = t1554 * t1586;
    let t38464 = t38463 * t2;
    let t38477 = t355 * t7241;
    let t38478 = t38477 * t2;
    let t38482 = t7760 * t369;
    (t38449, t38459, t38461, t38463, t38464, t38477, t38478, t38482)
}
