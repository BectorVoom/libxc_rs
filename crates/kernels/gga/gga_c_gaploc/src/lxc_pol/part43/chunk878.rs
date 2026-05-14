//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 878/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk878<F: Float>(t42168: F, t42170: F, t42172: F, t42173: F, t42184: F, t42188: F, t42189: F, t42194: F, t42198: F, t42200: F, t42203: F, t42205: F, t42208: F, t42221: F, t48069: F, t48071: F, t48073: F, t48074: F, t48076: F, t48081: F) -> (F,) {
    let t50891 = t48069 - t42168 - t48071 - t42170 + t48073 + t42172 + t42173 + t48074 - t48076 + t42184 - t42188 + 0.89376224879626066674e-1 * t42189 - t42194 + t42198 - t42200 - t42203 - t42205 - t42208 - 0.38342925953920749676e0 * t48081 - t42221;
    (t50891,)
}
