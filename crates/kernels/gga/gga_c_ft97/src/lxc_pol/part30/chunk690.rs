//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 690/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk690<F: Float>(t6962: F, t92: F, t1253: F, t6261: F, t193: F, t4299: F, t6353: F, t1476: F, t2665: F, t684: F, t2749: F, t7124: F) -> (F, F, F, F, F) {
    let t29008 = t6962 * t92;
    let t29016 = t6261 * t1253;
    let t29017 = t193 * t29016;
    let t29020 = t6353 * t4299;
    let t29024 = t1476 * t1253;
    let t29026 = t2665 * t29024 * t684;
    let t29030 = t2749 * t7124;
    (t29008, t29017, t29020, t29026, t29030)
}
