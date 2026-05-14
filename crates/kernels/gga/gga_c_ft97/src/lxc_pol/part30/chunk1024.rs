//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1024/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1024<F: Float>(t1882: F, t36188: F, t36142: F, t8392: F, t36232: F, t36130: F, t36250: F, t114271: F, t143621: F, t143924: F, t143953: F, t152780: F, t153560: F, t15460: F, t1901: F, t2862: F, t28847: F, t29128: F, t29129: F, t29189: F, t296: F, t319: F, t35833: F, t4176: F, t4181: F, t446: F, t44600: F, t53797: F, t7672: F, t882: F) -> (F, F, F, F) {
    let t154338 = t1882 * t36188;
    let t154357 = t8392 * t36142;
    let t154359 = t1882 * t36232;
    let t154362 = t1882 * t36130;
    let t154392 = t1882 * t36250;
    let t154394 = -4.0 / 9.0 * t154362 - 2.0 / 9.0 * t143924 + 4.0 / 3.0 * t446 * t2862 * t882 * t35833 + 4.0 / 3.0 * t446 * t2862 * t319 * t152780 + 4.0 / 9.0 * t53797 * t114271 * t29189 + 2.0 * t1901 * t15460 * t143621 * t4176 + 8.0 * t1901 * t29128 * t44600 * t7672 * t4181 - 4.0 * t1901 * t29128 * t29129 * t28847 + 2.0 / 3.0 * t446 * t296 * t153560 - 2.0 / 9.0 * t154392 - t143953;
    (t154338, t154357, t154359, t154394)
}
