//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 416/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk416<F: Float>(t1645: F, t1987: F, t121: F, t2084: F, t1: F, t313: F, t191: F, t835: F, t830: F, t106: F, t787: F) -> (F, F, F, F, F, F, F, F) {
    let t5974 = t1645 * t1987;
    let t6058 = t121 * t2084;
    let t6059 = t6058 * t1;
    let t6060 = t313 * t6059;
    let t6066 = t191 * t835;
    let t6109 = t830 * t1;
    let t6110 = t6109 * t106;
    let t6111 = t787 * t6110;
    (t5974, t6058, t6059, t6060, t6066, t6109, t6110, t6111)
}
