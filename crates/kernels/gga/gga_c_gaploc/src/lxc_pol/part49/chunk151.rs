//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 151/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk151<F: Float>(t304: F, t747: F, t178: F, t670: F, t108: F, t260: F, t14: F, t1: F, t271: F, t509: F, t110: F, t257: F, t667: F, t10: F, t107: F, t183: F, t266: F, t305: F, t306: F, t677: F) -> (F, F) {
    let t748 = t304 * t747;
    let t749 = t670 * t178;
    let t752 = t260 * t108;
    let t753 = t752 * t14;
    let t754 = t271 * t1;
    let t755 = t754 * t509;
    let t758 = t110 * t257;
    let t759 = t758 * t667;
    let t768 = 0.58998125e-2 * t749 * t306 - 0.11799625e-1 * t753 * t755 - 0.58998125e-2 * t305 * t759 - 0.14341111111111111111e-1 * t107 * t10 * t266 - 0.21511666666666666667e-1 * t107 * t183 * t677;
    (t748, t768)
}
