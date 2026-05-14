//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 977/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk977<F: Float>(t1526: F, t2666: F, t9483: F, t10227: F, t10215: F, t13598: F, t10223: F, t2252: F, t2644: F, t342: F, t10231: F, t630: F, t784: F, t8639: F, t11401: F, t443: F, t444: F) -> (F, F, F, F, F, F, F, F) {
    let t44666 = t1526 * t9483 * t2666;
    let t44669 = t1526 * t9483 * t10227;
    let t44672 = t1526 * t13598 * t10215;
    let t44683 = t1526 * t9483 * t10223;
    let t44709 = t342 * t2252 * t2644;
    let t44712 = t342 * t630 * t10231;
    let t44716 = 5.0 / 54.0 * t342 * t8639 * t784;
    let t46862 = t443 * t444 * t11401;
    (t44666, t44669, t44672, t44683, t44709, t44712, t44716, t46862)
}
