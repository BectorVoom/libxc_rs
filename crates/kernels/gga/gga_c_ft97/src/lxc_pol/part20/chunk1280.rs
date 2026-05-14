//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1280/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1280<F: Float>(t1882: F, t29309: F, t29350: F, t29250: F, t10443: F, t10683: F, t1091: F, t11593: F, t1255: F, t14607: F, t1508: F, t15133: F, t15460: F, t15477: F, t1901: F, t25000: F, t25004: F, t25272: F, t25474: F, t2862: F, t2874: F, t29067: F, t29128: F, t29129: F, t3281: F, t3746: F, t4176: F, t446: F, t6287: F, t6393: F, t72397: F, t835: F, t840: F, t99102: F, t99164: F, t99693: F) -> (F,) {
    let t114683 = 2.0 / 9.0 * t1882 * t29309;
    let t114694 = 4.0 / 9.0 * t1882 * t29350;
    let t114707 = 4.0 / 9.0 * t1882 * t29250;
    let t114719 = 4.0 / 3.0 * t446 * t2862 * t1255 * t25004 + 2.0 / 3.0 * t446 * t2862 * t1255 * t25000 - 2.0 * t446 * t10683 * t1508 * t15477 - t114683 + 2.0 / 3.0 * t99693 - t446 * t835 * t25474 * t1091 / 9.0 + 4.0 / 9.0 * t3281 * t835 * t6393 * t3746 - t114694 + 2.0 / 3.0 * t446 * t840 * t15133 * t6287 - 4.0 / 9.0 * t11593 * t10443 * t29067 + t1901 * t2874 * t99164 * t1091 / 9.0 - t114707 - 4.0 / 3.0 * t1901 * t72397 * t25272 - 2.0 * t1901 * t29128 * t29129 * t14607 - 4.0 / 3.0 * t1901 * t15460 * t99102 * t4176;
    (t114719,)
}
