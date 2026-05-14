//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1025/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1025<F: Float>(t1466: F, t25405: F, t681: F, t1476: F, t2892: F, t25452: F, t25462: F, t458: F, t6209: F, t6219: F, t25135: F, t2680: F, t2399: F, t6224: F, t25409: F, t6210: F) -> (F, F, F, F, F, F, F, F) {
    let t98357 = t1466 * t681 * t25405;
    let t98370 = t1476 * t2892;
    let t98380 = t25462 * t25452;
    let t98388 = t6209 * t458;
    let t98389 = t98388 * t6219;
    let t98407 = t2680 * t25135;
    let t98416 = t1466 * t2399 * t6224;
    let t98418 = t6210 * t25409;
    (t98357, t98370, t98380, t98388, t98389, t98407, t98416, t98418)
}
