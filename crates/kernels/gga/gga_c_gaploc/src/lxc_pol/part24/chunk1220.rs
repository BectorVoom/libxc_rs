//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1220/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1220<F: Float>(t21446: F, t2508: F, t3009: F, t7226: F, t21783: F, t1850: F, t29273: F, t29280: F, t32253: F, t32256: F, t32259: F, t32261: F, t32266: F, t32269: F, t32272: F, t32275: F, t32277: F, t32281: F, t5396: F) -> F {
    let t32285 = F::new(0.92286314761706691402e-1) * t2508 * t7226 * t3009 * t21446;
    let t32289 = F::new(0.46143157380853345701e-1) * t2508 * t7226 * t3009 * t21783;
    let t32290 = -t32253 - t32256 + t29273 - t29280 - t32259 - F::new(0.17090058289204942853e-2) * t1850 * t5396 * t32261 + t32266 + t32269 - t32272 + t32275 - t32277 - t32281 - t32285 - t32289;
    t32290
}
