//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 366/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk366<F: Float>(t2253: F, t634: F, t645: F, t422: F, t70: F, t1557: F, t179: F, t630: F, t41: F) -> (F, F, F, F, F) {
    let t2254 = t2253 * t634;
    let t2256 = t2253 * t645;
    let t2258 = t70 * t422;
    let t2259 = t179 * t1557;
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    (t2254, t2256, t2258, t2259, t2265)
}
