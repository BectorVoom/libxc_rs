//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 886/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk886<F: Float>(t1429: F, t42363: F, t42367: F, t42370: F, t42373: F, t42376: F, t42379: F, t42380: F, t42381: F, t42385: F, t42388: F, t42390: F, t42392: F, t42395: F, t42398: F, t42401: F, t42405: F, t42407: F, t42408: F, t42413: F, t42416: F, t549: F) -> F {
    let t42418 = F::new(0.87421871174939309263e2) * t42363 + t42367 + t42370 + t42373 - t42376 + t42379 - t42380 + t42381 - t42385 + t42388 - t42390 + t42392 - t42395 - t42398 - t42401 - t42405 + t42407 + F::new(0.39722766613167140743e-1) * t1429 * t549 * t42408 - t42413 + F::new(0.85206502119823888169e-1) * t42416;
    t42418
}
