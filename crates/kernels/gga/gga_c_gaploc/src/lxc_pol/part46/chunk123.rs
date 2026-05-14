//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 123/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk123<F: Float>(t475: F, t569: F, t568: F, t200: F, t8: F, t203: F, t61: F, t120: F, t196: F) -> (F, F, F, F) {
    let t575 = t569 * t475;
    let t576 = t568 * t575;
    let t579 = t8 * t200;
    let t580 = t579 * t203;
    let t581 = t61 * t580;
    let t584 = t196 * t120;
    (t576, t579, t581, t584)
}
