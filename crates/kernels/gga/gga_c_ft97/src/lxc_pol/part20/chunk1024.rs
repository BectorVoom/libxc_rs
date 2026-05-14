//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1024/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1024<F: Float>(t1466: F, t2399: F, t6266: F, t1465: F, t1771: F, t6219: F, t25462: F, t25467: F, t25448: F, t25479: F, t92: F, t25392: F, t681: F, t25471: F, t25396: F, t25401: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98309 = t1466 * t2399 * t6266;
    let t98317 = t1465 * t1771;
    let t98318 = t98317 * t6219;
    let t98322 = t25462 * t25467;
    let t98333 = t25462 * t25448;
    let t98335 = t25479 * t92;
    let t98341 = t1466 * t681 * t25392;
    let t98342 = t25462 * t25471;
    let t98351 = t1466 * t681 * t25396;
    let t98354 = t1466 * t681 * t25401;
    (t98309, t98317, t98318, t98322, t98333, t98335, t98341, t98342, t98351, t98354)
}
