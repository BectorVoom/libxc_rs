//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 585/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk585<F: Float>(t1725: F, t3085: F, t626: F, t934: F, t419: F, t3095: F, t8715: F, t122: F, t409: F, t371: F, t408: F, t929: F, t11174: F, t17: F, t355: F, t3001: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11296 = t1725 * t3085;
    let t11297 = 0.1134997482304526749e-1 * t11296;
    let t11298 = t626 * t934;
    let t11299 = t419 * t11298;
    let t11303 = t8715 * t3095;
    let t11304 = t419 * t11303;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11375 = t408 * t929;
    let t11401 = t11174 * t17;
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    (t11296, t11297, t11299, t11304, t11361, t11375, t11401, t11402, t11404)
}
