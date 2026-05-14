//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1172/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1172<F: Float>(t29682: F, t376: F, t89: F, t101603: F, t25883: F, t25893: F, t6495: F, t101633: F, t22952: F, t3266: F, t101710: F, t101712: F, t102212: F, t116679: F, t116683: F, t116688: F, t116692: F, t93505: F) -> (F, F, F, F) {
    let t116695 = t89 * t376 * t29682;
    let t116696 = 4.0 / 3.0 * t116695;
    let t116699 = t25893 * t101603 * t6495 * t25883;
    let t116703 = t22952 * t101633 * t6495 * t3266;
    let t116705 = 8.0 / 27.0 * t101710 - 4.0 / 9.0 * t101712 - t102212 + t93505 + t116679 / 3.0 + 4.0 * t116683 - 3.0 / 8.0 * t116688 - 6.0 * t116692 - t116696 + 3.0 / 2.0 * t116699 + 3.0 * t116703;
    (t116695, t116699, t116703, t116705)
}
