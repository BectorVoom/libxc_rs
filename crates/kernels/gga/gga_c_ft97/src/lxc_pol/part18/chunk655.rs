//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 655/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk655<F: Float>(t3057: F, t428: F, t374: F, t1685: F, t930: F, t25: F, t3099: F, t3066: F, t1655: F, t373: F, t35: F, t3064: F, t1711: F, t938: F, t371: F, t122: F, t409: F) -> (F, F, F, F, F, F, F, F) {
    let t11339 = t3057 * t428;
    let t11340 = t374 * t11339;
    let t11343 = t930 * t1685;
    let t11344 = t374 * t11343;
    let t11347 = t3099 * t25;
    let t11348 = t11347 * t3066;
    let t11351 = t373 * t1655;
    let t11352 = t11351 * t35;
    let t11353 = t3064 * t11352;
    let t11356 = t1711 * t938;
    let t11357 = t371 * t11356;
    let t11360 = t409 * t122;
    (t11340, t11344, t11348, t11352, t11353, t11356, t11357, t11360)
}
