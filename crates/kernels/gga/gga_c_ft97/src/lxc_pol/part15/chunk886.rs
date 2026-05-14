//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 886/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk886<F: Float>(t7954: F, t85516: F, t92: F, t85483: F, t7763: F, t85469: F, t1642: F, t85491: F, t1557: F, t85451: F, t85465: F, t7800: F, t378: F, t58969: F, t73956: F, t73958: F, t73983: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t85518 = t92 * t7954 * t85516;
    let t85522 = t92 * t7954 * t85483;
    let t85524 = t7763 * t85469;
    let t85526 = t92 * t1642 * t85524;
    let t85529 = t92 * t1642 * t85491;
    let t85531 = t1557 * t85451;
    let t85533 = t92 * t1642 * t85531;
    let t85536 = t92 * t1642 * t85465;
    let t85538 = t7800 * t85469;
    let t85540 = t92 * t378 * t85538;
    let t85542 = -8.0 / 9.0 * t58969 - 8.0 / 3.0 * t73956 + 8.0 / 9.0 * t73958 + 40.0 / 9.0 * t85518 + 40.0 / 81.0 * t73983 - 20.0 / 9.0 * t85522 - 8.0 * t85526 + 8.0 * t85529 - 2.0 / 3.0 * t85533 - 8.0 / 9.0 * t85536 + 8.0 * t85540;
    (t85518, t85522, t85524, t85526, t85529, t85531, t85533, t85536, t85538, t85540, t85542)
}
