//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 304/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk304<F: Float>(t1557: F, t1736: F, t1570: F, t422: F, t95: F, t96: F, t1542: F, t9: F) -> (F, F, F, F) {
    let t1737 = t1736 * t1557;
    let t1742 = t422 * t1570;
    let t1766 = F::new(1.0) / t96 / t95;
    let t1771 = t9 * t1542;
    (t1737, t1742, t1766, t1771)
}
