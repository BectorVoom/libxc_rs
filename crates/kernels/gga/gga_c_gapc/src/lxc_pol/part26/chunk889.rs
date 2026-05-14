//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 889/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk889<F: Float>(t4: F, t4054: F, t4048: F, t431: F, t122: F, t457: F, t4882: F, t1303: F, t521: F, t1338: F, t1: F, t4049: F, t172: F, t5963: F, t101: F, t1645: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13654 = t4054 * t4;
    let t13675 = t431 * t4048;
    let t13676 = t13675 * t122;
    let t13679 = t4054 * M_PI * t457;
    let t13736 = t4882 * t122;
    let t13738 = t521 * t1303;
    let t13790 = t521 * t1338;
    let t13850 = t4049 * t1;
    let t13853 = t5963 * t172;
    let t14541 = t1645 * t101;
    (t13654, t13675, t13676, t13679, t13736, t13738, t13790, t13850, t13853, t14541)
}
