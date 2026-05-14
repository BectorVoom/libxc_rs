//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1121/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1121<F: Float>(t42109: F, t6119: F, t109414: F, t13706: F, t109443: F, t109446: F, t109451: F, t109455: F, t109459: F, t109463: F, t109467: F, t109470: F, t109473: F, t109477: F, t109479: F, t108032: F, t108074: F, t108119: F, t108162: F, t108217: F, t108242: F, t108272: F, t108316: F, t108351: F, t108399: F, t108435: F, t109352: F, t109395: F, t109427: F, t109440: F, t762: F) -> (F, F) {
    let t109481 = t42109 * t6119;
    let t109483 = t109414 * t109481 * t13706;
    let t109485 = t109443 + t109446 / 3.0 + 2.0 / 9.0 * t109451 + t109455 / 3.0 - 12.0 * t109459 - 6.0 * t109463 - 12.0 * t109467 + t109470 - 4.0 / 3.0 * t109473 - t109477 + 4.0 / 3.0 * t109479 - 4.0 / 9.0 * t109483;
    let t109490 = t762 * (t108032 + t108074 + t108119 + t108162 + t108217 + t108242 + t108272 + t108316 + t108351 + t108399 + t108435 + t109352 + t109395 + t109427 + t109440 + t109485);
    (t109483, t109490)
}
