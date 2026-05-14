//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 556/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk556<F: Float>(t10846: F, t10874: F, t10902: F, t10945: F, t10991: F, t11036: F, t11087: F, t11122: F, t3511: F, t841: F, t3073: F, t977: F, t3556: F, t448: F, t2756: F, t999: F) -> (F, F, F, F, F) {
    let t11125 = t10846 + t10874 + t10902 + t10945 + t10991 + t11036 + t11087 + t11122;
    let t11127 = t3511 * t841;
    let t11135 = t3073 * t977;
    let t11154 = t3556 * t448;
    let t11157 = t999 * t2756;
    (t11125, t11127, t11135, t11154, t11157)
}
