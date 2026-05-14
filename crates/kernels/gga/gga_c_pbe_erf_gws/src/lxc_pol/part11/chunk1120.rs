//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1120<F: Float>(t49316: F, t49318: F, t49327: F, t49329: F, t49334: F, t49344: F, t49345: F, t49347: F, t49356: F, t49362: F, t49364: F, t49371: F, t49372: F, t49378: F, t49382: F, t49387: F, t49388: F, t49399: F, t49415: F, t49471: F, t49472: F, t49478: F) -> (F, F) {
    let t50565 = -t49316 - t49318 - t49327 - t49329 - t49334 + t49344 - t49345 - t49347 - t49356 - t49362 + t49364;
    let t50567 = t49371 + t49372 - t49378 + t49382 + t49387 + t49388 + t49399 + t49415 - t49471 - t49472 - t49478;
    (t50565, t50567)
}
