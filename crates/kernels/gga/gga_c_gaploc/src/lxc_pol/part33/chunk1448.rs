//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1448/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1448<F: Float>(t12306: F, t1935: F, t29304: F, t29310: F, t32272: F, t32275: F, t32277: F, t32281: F, t32285: F, t32289: F, t32329: F, t32332: F, t32334: F, t32337: F, t3723: F, t681: F) -> F {
    let t39383 = -t32272 + t32275 - t32277 - t32281 - t32285 - t32289 + t29304 + F::new(0.76905262301422242837e-2) * t1935 * t3723 + F::new(0.15381052460284448567e-1) * t681 * t12306 + t29310 - t32329 - t32332 + t32334 + t32337;
    t39383
}
