//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 409/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk409<F: Float>(t379: F, t643: F, t2266: F, t1570: F, t179: F, t1559: F, t72: F, t1580: F, t632: F, t178: F) -> (F, F, F, F, F, F) {
    let t2267 = t379 * t643;
    let t2268 = t2266 * t2267;
    let t2271 = t179 * t1570;
    let t2273 = t72 * t2271 * t1559;
    let t2277 = t72 * t632 * t1580;
    let t2280 = t178 * t178;
    let t2281 = 1.0 / t2280;
    (t2268, t2271, t2273, t2277, t2280, t2281)
}
