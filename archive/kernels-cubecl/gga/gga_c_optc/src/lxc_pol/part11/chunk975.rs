//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 975/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk975<F: Float>(t17780: F, t17851: F, t241: F, t17429: F, t17431: F, t17433: F, t17435: F, t17438: F, t17527: F, t17645: F, t17655: F, t17658: F, t17750: F, t17753: F) -> (F, F) {
    let t17853 = t241 * (t17780 + t17851);
    let t17854 = t17429 + t17431 + t17433 + t17435 - t17438 - t17527 + t17750 - t17655 + t17658 - t17753 - t17645 + t17853;
    (t17853, t17854)
}
