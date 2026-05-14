//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 805/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk805<F: Float>(t1564: F, t38283: F, t446: F, t1546: F, t7746: F, t89: F, t1555: F, t37357: F, t7764: F, t37422: F, t37424: F, t37427: F, t37433: F, t38257: F, t38261: F, t38266: F, t38271: F, t38275: F, t38279: F, t38281: F) -> (F, F, F, F) {
    let t38285 = t446 * t1564 * t38283;
    let t38288 = t89 * t1546 * t7746;
    let t38292 = t89 * t1555 * t7764 * t37357;
    let t38294 = t37422 + 2.0 / 9.0 * t37424 + 4.0 / 3.0 * t37427 + 4.0 * t37433 - t38257 / 6.0 - t38261 + 4.0 / 3.0 * t38266 - 4.0 / 9.0 * t38271 - 8.0 / 9.0 * t38275 - 2.0 / 3.0 * t38279 - 2.0 / 9.0 * t38281 - 2.0 / 3.0 * t38285 + 2.0 / 27.0 * t38288 - 4.0 / 3.0 * t38292;
    (t38285, t38288, t38292, t38294)
}
