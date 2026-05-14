//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1106/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1106<F: Float>(t10346: F, t11683: F, t23305: F, t2440: F, t22657: F, t2456: F, t11636: F, t11684: F, t6940: F, t10123: F, t10243: F, t2531: F, t329: F, t827: F, t6210: F, t959: F) -> (F, F, F, F, F) {
    let t35776 = t10346 * t23305 * t11683 * t2440;
    let t35780 = t10346 * t22657 * t11683 * t2456;
    let t35783 = t11636 * t6940 * t11684;
    let t35788 = t10243 * t827 * t10123 * t329 * t2531;
    let t35790 = t6210 * t959;
    (t35776, t35780, t35783, t35788, t35790)
}
