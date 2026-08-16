//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 372/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk372<F: Float>(t169: F, t3116: F, t172: F, t452: F, t2321: F, t894: F, t882: F, t203: F, t3086: F, t492: F, t2334: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t3117 = t3116 * t169;
    let t3118 = t3117 * t172;
    let t3119 = t452 * t3118;
    let t3122 = t894 * t2321;
    let t3124 = F::cast_from(0.23712505529730124666e-2_f64) * t882 * t3122;
    let t3125 = t3086 * t203;
    let t3126 = t492 * t3125;
    let t3129 = t883 * t2334;
    (t3118, t3119, t3122, t3124, t3125, t3126, t3129)
}
