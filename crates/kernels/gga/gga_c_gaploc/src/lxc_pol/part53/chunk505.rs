//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 505/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk505<F: Float>(t3228: F, t471: F, t64: F, t9664: F, t9666: F, t9674: F, t9676: F, t9678: F, t9682: F) -> (F,) {
    let t9688 = t9678 * t471 - 4.0 / 3.0 * t3228 * t64 + t9682 / 2.0 - 7.0 / 512.0 * t9664 + 21.0 / 16384.0 * t9666 - 7.0 / 16384.0 * t9674 + 7.0 / 1536.0 * t9676;
    (t9688,)
}
