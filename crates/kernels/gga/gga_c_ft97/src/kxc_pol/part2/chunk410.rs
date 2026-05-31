//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 410/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk410<F: Float>(t379: F, t643: F, t2266: F, t1570: F, t179: F, t1559: F, t72: F, t1580: F, t632: F, t178: F, t637: F, t1638: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2267 = t379 * t643;
    let t2268 = t2266 * t2267;
    let t2271 = t179 * t1570;
    let t2273 = t72 * t2271 * t1559;
    let t2277 = t72 * t632 * t1580;
    let t2280 = t178 * t178;
    let t2281 = F::cast_from(1.0_f64) / t2280;
    let t2282 = t643 * t643;
    let t2284 = t637 * t2281 * t2282;
    let t2289 = F::cast_from(0.19257444444444444444e0_f64) * t1638;
    (t2267, t2268, t2273, t2277, t2280, t2281, t2282, t2284, t2289)
}
