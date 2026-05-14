//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 980/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk980<F: Float>(t1476: F, t4311: F, t840: F, t7033: F, t8392: F, t1901: F, t24955: F, t24960: F, t29199: F, t29204: F, t29209: F, t29212: F, t29216: F, t29219: F, t29223: F, t29226: F, t29229: F, t29232: F, t446: F) -> (F, F, F) {
    let t29235 = t840 * t4311 * t1476;
    let t29238 = t8392 * t7033;
    let t29241 = -2.0 / 9.0 * t1901 * t29199 - 2.0 / 9.0 * t1901 * t29204 + 2.0 / 27.0 * t1901 * t29209 - 2.0 / 9.0 * t1901 * t29212 - t1901 * t29216 / 9.0 - 2.0 / 9.0 * t1901 * t29219 - t1901 * t29223 / 9.0 - 2.0 / 9.0 * t1901 * t29226 + 2.0 / 27.0 * t1901 * t29229 + t29232 / 9.0 - t446 * t29235 / 3.0 - t24955 - t29238 / 27.0 + t24960 / 9.0;
    (t29235, t29238, t29241)
}
