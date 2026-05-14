//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 631/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk631<F: Float>(t4299: F, t6353: F, t1253: F, t1476: F, t2665: F, t684: F, t2749: F, t7124: F, t1212: F, t880: F, t6222: F, t193: F, t28719: F, t798: F, t317: F, t4246: F, t6386: F) -> (F, F, F, F, F, F, F, F) {
    let t29020 = t6353 * t4299;
    let t29024 = t1476 * t1253;
    let t29026 = t2665 * t29024 * t684;
    let t29030 = t2749 * t7124;
    let t29033 = t880 * t1212;
    let t29034 = t6222 * t29033;
    let t29035 = t193 * t29034;
    let t29040 = t798 * t28719;
    let t29041 = t29040 * t317;
    let t29042 = t193 * t29041;
    let t29045 = t4246 * t6386;
    (t29020, t29026, t29030, t29033, t29035, t29040, t29042, t29045)
}
