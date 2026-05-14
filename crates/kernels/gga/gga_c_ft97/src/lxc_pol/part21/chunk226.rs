//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 226/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk226<F: Float>(t1528: F, t363: F, t432: F, t72: F, t1524: F, t1526: F, t1527: F, t342: F, t343: F, t438: F, t14: F, t360: F, t12: F, t10: F, t83: F, t355: F, t375: F) -> (F, F, F, F, F, F, F, F) {
    let t1529 = t1528 * t363;
    let t1533 = t72 * t432;
    let t1537 = t438 - t1524 - t1526 * t1527 * t1529 / 12.0 - t342 * t343 * t1533 / 4.0;
    let t1541 = 1.0 / t14 / t360;
    let t1542 = t12 * t1541;
    let t1544 = t10 * t1542 * t83;
    let t1545 = 2.0 / 27.0 * t1544;
    let t1546 = t375 * t355;
    (t1529, t1533, t1537, t1541, t1542, t1544, t1545, t1546)
}
