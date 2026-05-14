//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 720/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk720<F: Float>(t16751: F, t2: F, t4714: F, t1985: F, t558: F, t4668: F, t9016: F, t3408: F, t3518: F, t16395: F, t582: F, t1775: F, t4765: F, t4768: F, t4759: F, t458: F, t4776: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17251 = 2.0 / 27.0 * t16751;
    let t17254 = t2 * t4714;
    let t17256 = t1985 * t17254 * t558;
    let t17259 = t2 * t4668;
    let t17261 = t9016 * t17259 * t558;
    let t17265 = t1985 * t3518 * t3408;
    let t17268 = t582 * t16395;
    let t17272 = t1775 * t4765;
    let t17274 = t1775 * t4768;
    let t17276 = t1775 * t4759;
    let t17279 = t458 * t4776;
    (t17251, t17256, t17261, t17265, t17268, t17272, t17274, t17276, t17279)
}
