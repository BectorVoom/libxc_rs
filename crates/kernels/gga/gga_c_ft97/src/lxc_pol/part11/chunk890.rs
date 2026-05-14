//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 890/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk890<F: Float>(t40301: F, t40306: F, t40309: F, t40312: F, t40315: F, t40318: F, t40321: F, t40490: F, t40494: F, t40497: F, t40500: F, t40503: F, t40506: F, t40509: F, t40512: F, t40611: F, t40627: F, t40644: F) -> (F,) {
    let t40660 = -16.0 / 27.0 * t40301 + 8.0 / 3.0 * t40306 - 8.0 / 3.0 * t40309 - 8.0 / 27.0 * t40312 - 16.0 / 81.0 * t40315 + 4.0 / 27.0 * t40318 + 40.0 / 243.0 * t40321 + t40490 / 6.0 - t40494 / 4.0 + 112.0 / 243.0 * t40497 + 16.0 / 27.0 * t40500 - 8.0 / 9.0 * t40503 + 4.0 / 9.0 * t40506 + 8.0 / 3.0 * t40509 + 16.0 / 9.0 * t40512;
    let t40662 = t40611 + t40627 + t40644 + t40660;
    (t40662,)
}
