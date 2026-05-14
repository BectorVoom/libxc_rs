//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1075/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1075<F: Float>(t2399: F, t6109: F, t6879: F, t108057: F, t2354: F, t446: F, t108322: F, t108326: F, t108330: F, t108334: F, t108338: F, t108342: F, t97030: F, t97061: F, t97356: F, t27832: F, t681: F, t89: F) -> (F, F, F, F) {
    let t108345 = t6109 * t2399 * t6879;
    let t108346 = t108345 / 9.0;
    let t108348 = t446 * t2354 * t108057;
    let t108351 = t97030 - 6.0 * t108322 + 4.0 * t108326 - 2.0 / 3.0 * t108330 - t108334 + 4.0 * t108338 + 2.0 * t108342 + t108346 + 2.0 * t108348 + t97356 + 16.0 / 9.0 * t97061;
    let t108353 = t89 * t681 * t27832;
    (t108345, t108348, t108351, t108353)
}
