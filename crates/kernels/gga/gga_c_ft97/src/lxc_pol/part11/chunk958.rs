//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 958/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk958<F: Float>(t2581: F, t8232: F, t10055: F, t1882: F, t10159: F, t192: F, t33300: F, t9819: F, t2528: F, t255: F, t42123: F, t10031: F, t10143: F, t10007: F, t10039: F, t10044: F, t10075: F, t13885: F, t14200: F, t1901: F, t2373: F, t2409: F, t2413: F, t242: F, t2568: F, t2569: F, t2574: F, t2579: F, t2619: F, t41414: F, t41435: F, t446: F, t713: F, t724: F, t761: F, t773: F, t9787: F) -> (F, F, F, F, F, F, F, F) {
    let t42491 = t8232 * t2581;
    let t42493 = t1882 * t10055;
    let t42498 = t1882 * t10159;
    let t42500 = t192 * t33300;
    let t42509 = t1882 * t9819;
    let t42511 = t8232 * t2528;
    let t42517 = t42123 * t255;
    let t42546 = t1882 * t10031;
    let t42557 = t1882 * t10143;
    let t42563 = -8.0 * t1901 * t13885 * t761 * t713 * t10044 - 8.0 / 3.0 * t1901 * t9787 * t10075 + 8.0 / 3.0 * t1901 * t10007 * t2409 * t2579 + 8.0 / 9.0 * t1901 * t14200 * t41435 - 2.0 / 3.0 * t446 * t724 * t2619 * t2413 - 12.0 * t446 * t242 * t41414 + 8.0 / 3.0 * t42546 + 8.0 * t446 * t2574 * t2568 * t2373 * t2569 + 8.0 * t446 * t2574 * t773 * t10039 - 8.0 / 9.0 * t42557 + 4.0 / 3.0 * t446 * t724 * t2619 * t2409;
    (t42491, t42493, t42498, t42500, t42509, t42511, t42517, t42563)
}
