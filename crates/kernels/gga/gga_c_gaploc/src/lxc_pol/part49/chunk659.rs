//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 659/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk659<F: Float>(t12156: F, t12191: F, t12199: F, t12202: F, t12235: F, t12247: F, t12249: F, t12267: F, t3749: F, t841: F, t3730: F, t747: F, t12166: F, t738: F, t12255: F, t740: F) -> (F, F, F, F, F) {
    let t12270 = t12156 + t12191 + t12199 + t12202 + t12235 + t12247 + t12249 + t12267;
    let t12272 = t3749 * t841;
    let t12277 = t3730 * t747;
    let t12281 = t738 * t12166;
    let t12284 = t12255 * t740;
    (t12270, t12272, t12277, t12281, t12284)
}
