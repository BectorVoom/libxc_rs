//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 659/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk659<F: Float>(t18: F, t432: F, t1903: F, t1902: F, t492: F, t1910: F, t1909: F, t363: F, t3187: F, t1882: F, t3277: F, t3273: F, t1853: F, t942: F, t1852: F, t452: F) -> (F, F, F, F, F, F, F) {
    let t11594 = t18 * t432;
    let t11595 = t1903 * t11594;
    let t11596 = t1902 * t11595;
    let t11599 = t18 * t492;
    let t11600 = t1910 * t11599;
    let t11601 = t1909 * t11600;
    let t11604 = t18 * t363;
    let t11605 = t3187 * t11604;
    let t11606 = t1909 * t11605;
    let t11610 = 2.0 / 27.0 * t1882 * t3277;
    let t11612 = 2.0 / 9.0 * t1882 * t3273;
    let t11613 = t942 * t1853;
    let t11615 = t452 * t1852 * t11613;
    (t11596, t11601, t11604, t11606, t11610, t11612, t11615)
}
