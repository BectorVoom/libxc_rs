//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1013/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1013<F: Float>(t13605: F, t1526: F, t21123: F, t21125: F, t21181: F, t21399: F, t21442: F, t21457: F, t231: F, t2320: F, t342: F, t343: F, t3806: F, t42293: F, t42307: F, t69073: F, t69137: F, t81955: F, t81958: F) -> (F,) {
    let t89656 = -t342 * t343 * t231 * t21399 / 4.0 + t21123 - t42293 - t1526 * t3806 * t21442 / 3.0 - 7.0 / 27.0 * t1526 * t13605 * t42307 * t21181 - t1526 * t2320 * t21457 / 4.0 - t81955 / 9.0 - t81958 / 6.0 + t69073 / 6.0 + t69137 / 18.0 + 2.0 * t21125;
    (t89656,)
}
