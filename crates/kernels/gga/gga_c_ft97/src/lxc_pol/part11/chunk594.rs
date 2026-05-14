//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 594/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk594<F: Float>(t2268: F, t8675: F, t2253: F, t2273: F, t2281: F, t71: F, t2282: F, t379: F, t1647: F, t643: F, t2266: F, t118: F, t7911: F, t7899: F, t2007: F, t383: F) -> (F, F, F, F, F, F, F, F) {
    let t8676 = t8675 * t2268;
    let t8678 = t2253 * t2273;
    let t8680 = t71 * t2281;
    let t8681 = t379 * t2282;
    let t8682 = t8680 * t8681;
    let t8685 = t1647 * t643;
    let t8686 = t2266 * t8685;
    let t8690 = 1.0 / t118 / t7911;
    let t8691 = t8690 * t7899;
    let t8693 = t2007 * t383;
    (t8676, t8678, t8680, t8682, t8686, t8690, t8691, t8693)
}
