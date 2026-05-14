//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1332/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1332<F: Float>(t1882: F, t31938: F, t31837: F, t113956: F, t113968: F, t114792: F, t125541: F, t125544: F, t125663: F, t1501: F, t15369: F, t19002: F, t19006: F, t1901: F, t19240: F, t19399: F, t19630: F, t24908: F, t2874: F, t29071: F, t29082: F, t296: F, t4162: F, t446: F, t4973: F, t53797: F, t54032: F, t6360: F, t840: F, t871: F, t98966: F) -> (F,) {
    let t126541 = t1882 * t31938;
    let t126543 = t1882 * t31837;
    let t126562 = 4.0 / 3.0 * t446 * t296 * t125663 + t1901 * t2874 * t24908 * t4973 / 9.0 + t446 * t840 * t871 * t1501 * t19240 / 3.0 - 2.0 * t446 * t296 * t125541 - 2.0 * t446 * t296 * t125544 + 2.0 / 3.0 * t126541 - 4.0 / 9.0 * t126543 - t113956 - 4.0 / 3.0 * t1901 * t15369 * t29082 * t4162 + 4.0 / 9.0 * t53797 * t98966 * t19630 + 4.0 / 9.0 * t53797 * t114792 * t19002 - 4.0 / 27.0 * t54032 * t114792 * t19006 - t113968 + 2.0 * t1901 * t29071 * t6360 * t19399;
    (t126562,)
}
