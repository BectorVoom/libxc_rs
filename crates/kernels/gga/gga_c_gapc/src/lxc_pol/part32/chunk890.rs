//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 890/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk890<F: Float>(t11280: F, t11282: F, t11284: F, t11286: F, t11288: F, t11290: F, t11293: F, t11296: F, t11297: F, t11300: F, t11610: F, t12004: F, t224: F, t3797: F, t987: F, t3707: F, t435: F) -> (F, F, F, F) {
    let t12005 = -t11280 + t11282 + t11284 - t11286 + t11288 - t11290 + t11293 - t11296 + t11297 - t11300 + t11610;
    let t12006 = t12004 + t12005;
    let t12007 = t224 * t12006;
    let t12658 = t987 * t3797;
    let t12744 = t435 * t3707;
    (t12006, t12007, t12658, t12744)
}
