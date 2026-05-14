//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 624/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk624<F: Float>(t1775: F, t2109: F, t2098: F, t2114: F, t458: F, t582: F, t8307: F, t3506: F, t7789: F, t2: F, t9132: F, t9074: F, t2102: F, t9078: F, t143: F, t7760: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9205 = t1775 * t2109;
    let t9207 = t1775 * t2098;
    let t9209 = t458 * t2114;
    let t9211 = t582 * t8307;
    let t9214 = t3506 * t7789;
    let t9217 = t9132 * t2;
    let t9218 = t9217 * t9074;
    let t9221 = t2102 * t9078;
    let t9224 = t7760 * t143;
    (t9205, t9207, t9209, t9211, t9214, t9217, t9218, t9221, t9224)
}
