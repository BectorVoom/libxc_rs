//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1316/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1316<F: Float>(t31572: F, t684: F, t312: F, t31551: F, t10447: F, t112920: F, t113847: F, t114820: F, t15312: F, t15369: F, t15460: F, t18123: F, t1901: F, t19373: F, t19378: F, t19423: F, t19430: F, t19435: F, t19507: F, t19522: F, t25271: F, t25368: F, t2874: F, t2881: F, t29055: F, t29128: F, t29129: F, t31709: F, t31814: F, t31841: F, t44523: F, t4973: F, t56418: F, t6360: F) -> (F, F) {
    let t125826 = t31572 * t684;
    let t125847 = t312 * t31551;
    let t125852 = -2.0 / 3.0 * t1901 * t15460 * t25271 * t19373 - 2.0 * t1901 * t29128 * t29129 * t19378 + 4.0 / 3.0 * t1901 * t15369 * t29055 * t19423 + 2.0 * t1901 * t15460 * t113847 * t19430 - 4.0 / 3.0 * t1901 * t15460 * t29055 * t19435 - 2.0 / 9.0 * t1901 * t15312 * t31841 * t684 + 2.0 / 9.0 * t1901 * t44523 * t31814 * t684 + 2.0 / 3.0 * t1901 * t56418 * t125826 - 4.0 / 3.0 * t1901 * t112920 * t19507 - 4.0 / 3.0 * t1901 * t114820 * t19522 + t1901 * t10447 * t31709 / 9.0 + t1901 * t2881 * t25368 * t4973 / 9.0 + t1901 * t2881 * t6360 * t18123 / 9.0 + t1901 * t2874 * t125847 * t684 / 9.0;
    (t125826, t125852)
}
