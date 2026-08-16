//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2090/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2090<F: Float>(t41083: F, t789: F, t41011: F, t9561: F, t154: F, t1891: F, t205: F, t792: F, t9558: F, t118: F, t794: F, t9458: F) -> (F, F, F, F, F) {
    let t41156 = t41083 * t789;
    let t41158 = t41011 * t9561;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    let t41173 = t41170 * t118 * t794 * t9458;
    (t41156, t41158, t41160, t41161, t41173)
}
