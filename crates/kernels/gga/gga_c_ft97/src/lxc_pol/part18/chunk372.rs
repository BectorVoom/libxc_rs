//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 372/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk372<F: Float>(t1557: F, t179: F, t1559: F, t2258: F, t630: F, t70: F, t41: F, t639: F, t71: F) -> (F, F, F) {
    let t2259 = t179 * t1557;
    let t2261 = t2258 * t2259 * t1559;
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    let t2266 = t71 * t639;
    (t2261, t2265, t2266)
}
