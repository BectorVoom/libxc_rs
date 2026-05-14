//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 613/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk613<F: Float>(t1969: F, t9041: F, t446: F, t2075: F, t379: F, t1642: F, t525: F, t1643: F, t558: F, t143: F, t7800: F, t7765: F, t1555: F, t89: F, t1882: F, t1971: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9042 = t1969 * t9041;
    let t9043 = t446 * t9042;
    let t9045 = t379 * t2075;
    let t9046 = t1969 * t9045;
    let t9047 = t446 * t9046;
    let t9049 = t1642 * t525;
    let t9050 = t1643 * t558;
    let t9051 = t9049 * t9050;
    let t9052 = t446 * t9051;
    let t9054 = t143 * t7800;
    let t9055 = t9054 * t7765;
    let t9057 = t89 * t1555 * t9055;
    let t9059 = t1882 * t1971;
    (t9042, t9043, t9045, t9046, t9047, t9049, t9050, t9051, t9052, t9054, t9055, t9057, t9059)
}
