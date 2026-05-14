//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 680/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk680<F: Float>(t11021: F, t11023: F, t11025: F, t11019: F, t11036: F, t7775: F, t7778: F, t7782: F, t7820: F, t8192: F, t8195: F, t11043: F, t11392: F, t24: F, t469: F, t3155: F, t458: F) -> (F, F, F, F) {
    let t11646 = 2.0 / 27.0 * t11021;
    let t11647 = 4.0 / 27.0 * t11023;
    let t11648 = 4.0 / 81.0 * t11025;
    let t11656 = t11019 / 9.0 - t11646 - t11647 + t11648 - 8.0 / 81.0 * t7775 + t7778 / 27.0 + 2.0 / 81.0 * t7782 - 2.0 / 27.0 * t7820 - 8.0 / 27.0 * t8192 + t8195 / 9.0 - 2.0 / 27.0 * t11036;
    let t11659 = 4.0 / 81.0 * t11043;
    let t11665 = t24 * t469 * t11392;
    let t11668 = 2.0 / 3.0 * t458 * t3155;
    (t11656, t11659, t11665, t11668)
}
