//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1075/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1075<F: Float>(t26306: F, t8392: F, t1882: F, t26487: F, t26249: F, t47660: F, t5717: F, t26461: F, t26246: F, t12001: F, t26330: F, t1332: F, t7800: F, t47667: F, t101: F, t23249: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t103211 = 2.0 / 27.0 * t8392 * t26306;
    let t103216 = 2.0 / 9.0 * t1882 * t26487;
    let t103219 = 2.0 / 27.0 * t8392 * t26249;
    let t103252 = t47660 * t5717;
    let t103305 = 2.0 / 9.0 * t1882 * t26461;
    let t103343 = 2.0 / 27.0 * t8392 * t26246;
    let t103350 = t12001 * t26330;
    let t103423 = t1332 * t7800;
    let t103435 = t47667 * t5717;
    let t103439 = t101 * t23249;
    (t103211, t103216, t103219, t103252, t103305, t103343, t103350, t103423, t103435, t103439)
}
