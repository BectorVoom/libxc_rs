//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 605/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk605<F: Float>(t10782: F, t723: F, t2580: F, t3448: F, t7137: F, t3431: F, t795: F) -> (F, F, F, F) {
    let t10783 = t10782 * t723;
    let t10784 = t2580 * t10783;
    let t10788 = 0.20508069947045931423e-1 * t7137 * t3448;
    let t10789 = t795 * t3431;
    (t10783, t10784, t10788, t10789)
}
