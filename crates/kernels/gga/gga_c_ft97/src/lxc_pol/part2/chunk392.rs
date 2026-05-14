//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 392/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk392<F: Float>(t379: F, t643: F, t2266: F, t1570: F, t179: F, t1559: F, t72: F, t1580: F, t632: F, t178: F, t637: F, t1638: F, t1640: F, t1645: F, t1649: F, t1653: F, t2008: F, t2011: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2267 = t379 * t643;
    let t2268 = t2266 * t2267;
    let t2271 = t179 * t1570;
    let t2273 = t72 * t2271 * t1559;
    let t2277 = t72 * t632 * t1580;
    let t2280 = t178 * t178;
    let t2281 = 1.0 / t2280;
    let t2282 = t643 * t643;
    let t2284 = t637 * t2281 * t2282;
    let t2289 = 0.19257444444444444444e0 * t1638;
    let t2294 = -0.117377e0 * t2008 + 0.234754e0 * t2011 + t2289 + 0.9628722222222222222e-1 * t1640 - 0.9628722222222222222e-1 * t1645 + 0.28886166666666666666e0 * t1649 - 0.14443083333333333333e0 * t1653;
    (t2267, t2268, t2273, t2277, t2280, t2281, t2282, t2284, t2289, t2294)
}
