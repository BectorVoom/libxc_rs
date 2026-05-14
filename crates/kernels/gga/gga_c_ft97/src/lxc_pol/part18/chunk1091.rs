//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1091/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1091<F: Float>(t5577: F, t5580: F, t92557: F, t22632: F, t22751: F, t5590: F, t5592: F, t625: F, t5587: F, t3076: F, t38241: F, t6: F, t8: F, t22626: F, t409: F, t64: F) -> (F, F, F, F, F, F) {
    let t92559 = t5577 * t92557 * t5580;
    let t92571 = t5577 * t22632 * t22751;
    let t92574 = t5590 * t625 * t5592;
    let t92575 = t5587 * t92574;
    let t92579 = t3076 * t38241 * t6 * t8;
    let t92621 = t409 * t22626;
    let t92622 = t64 * t92621;
    (t92559, t92571, t92574, t92575, t92579, t92622)
}
