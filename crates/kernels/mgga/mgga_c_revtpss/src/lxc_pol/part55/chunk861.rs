//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 861/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk861<F: Float>(t13272: F, t7342: F, t2048: F, t26180: F, t26185: F, t26187: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28141: F, t28598: F, t28600: F, t6960: F, t7343: F, t7352: F, t7706: F, t7709: F) -> (F,) {
    let t28602 = t13272 * t7342;
    let t28621 = 40.0 / 9.0 * t26180 + 16.0 / 9.0 * t26185 + 40.0 / 9.0 * t28598 + 16.0 / 9.0 * t28600 - 5.0 / 3.0 * t28602 * t6960 - 2.0 / 3.0 * t28141 * t2048 - 5.0 / 3.0 * t26187 * t7706 - 5.0 / 3.0 * t7343 * t28105 - 5.0 / 3.0 * t7343 * t28109 - 2.0 / 3.0 * t28112 * t2048 - 2.0 / 3.0 * t28116 * t2048 - 2.0 / 3.0 * t28119 * t2048 - 2.0 / 3.0 * t7709 * t7352;
    (t28621,)
}
