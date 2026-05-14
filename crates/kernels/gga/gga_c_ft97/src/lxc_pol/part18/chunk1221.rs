//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1221/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1221<F: Float>(t101687: F, t101689: F, t101691: F, t101676: F, t101681: F, t101696: F, t93456: F, t93458: F, t93474: F, t93478: F, t93481: F, t93776: F, t101708: F, t101710: F, t101718: F, t101701: F, t101706: F, t101712: F, t101716: F, t101724: F, t101729: F, t101737: F, t93504: F, t93508: F) -> (F, F) {
    let t102202 = t101687 / 3.0;
    let t102203 = t101689 / 18.0;
    let t102204 = 2.0 / 27.0 * t101691;
    let t102206 = t93776 + t93456 - 8.0 / 9.0 * t93458 - 2.0 / 9.0 * t101676 + 4.0 * t101681 + 16.0 / 9.0 * t93474 - t93478 - t93481 + t102202 + t102203 + t102204 - 2.0 / 3.0 * t101696;
    let t102209 = 2.0 / 9.0 * t101708;
    let t102210 = 4.0 / 27.0 * t101710;
    let t102212 = 2.0 / 9.0 * t101718;
    let t102217 = t101701 / 3.0 + 2.0 / 9.0 * t101706 - t102209 + t102210 - 22.0 / 9.0 * t101712 - t101716 - t102212 + 4.0 / 27.0 * t93504 + t93508 + t101724 / 3.0 - t101729 / 2.0 + t101737 / 4.0;
    (t102206, t102217)
}
