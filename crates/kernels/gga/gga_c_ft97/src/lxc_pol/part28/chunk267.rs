//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 267/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk267<F: Float>(t630: F, t70: F, t41: F, t639: F, t71: F, t178: F, t1638: F, t1537: F, t947: F, t1546: F, t89: F, t921: F) -> (F, F, F, F, F, F) {
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    let t2266 = t71 * t639;
    let t2280 = t178 * t178;
    let t2281 = F::new(1.0) / t2280;
    let t2289 = F::new(0.19257444444444444444e0) * t1638;
    let t2976 = t1537 * t947;
    let t2981 = t89 * t1546 * t921;
    (t2265, t2266, t2281, t2289, t2976, t2981)
}
