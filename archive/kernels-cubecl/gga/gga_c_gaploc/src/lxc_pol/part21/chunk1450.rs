//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1450/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1450<F: Float>(t29273: F, t29280: F, t32201: F, t32207: F, t32213: F, t32222: F, t32226: F, t32241: F, t32243: F, t32245: F, t32253: F, t32256: F, t32259: F, t32266: F, t32269: F) -> F {
    let t39376 = t32201 + t32207 + t32213 + t32222 + t32226 + t32241 + t32243 + t32245 - t32253 - t32256 + t29273 - t29280 - t32259 + t32266 + t32269;
    t39376
}
