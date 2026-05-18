//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 435/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk435<F: Float>(t159: F, t2141: F, t104: F, t751: F, t14: F, t260: F, t445: F, t348: F, t19: F, t269: F, t1355: F, t257: F) -> (F, F, F, F, F) {
    let t2346 = t2141 * t159;
    let t2349 = t751 * t104;
    let t2350 = t2349 * t14;
    let t2355 = t260 * t445;
    let t2356 = t2355 * t348;
    let t2357 = t269 * t19;
    let t2358 = t2357 * t1355;
    let t2361 = t14 * t257;
    (t2346, t2350, t2356, t2358, t2361)
}
