//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2222/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2222<F: Float>(t2403: F, t5720: F, t5723: F, t17246: F, t699: F, t17249: F, t17252: F, t5717: F, t17255: F, t17279: F, t17240: F, t17243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t60168 = t2403 * t5720;
    let t60173 = t2403 * t5723;
    let t60192 = t699 * t17246;
    let t60194 = t699 * t17249;
    let t60202 = t699 * t17252;
    let t60204 = t2403 * t5717;
    let t60274 = t699 * t17255;
    let t60308 = t699 * t17279;
    let t60310 = t699 * t17240;
    let t60312 = t699 * t17243;
    (t60168, t60173, t60192, t60194, t60202, t60204, t60274, t60308, t60310, t60312)
}
