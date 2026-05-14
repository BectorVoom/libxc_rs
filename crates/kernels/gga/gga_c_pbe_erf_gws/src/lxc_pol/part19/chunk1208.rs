//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1208/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1208<F: Float>(t55562: F, t55564: F, t55569: F, t55570: F, t55572: F, t57108: F, t57110: F, t57112: F, t57114: F, t57117: F, t57119: F, t57121: F, t57123: F, t54293: F, t54294: F, t54305: F, t55580: F, t57127: F, t57130: F, t57132: F, t57134: F, t57138: F, t57140: F, t57142: F, t57144: F, t57146: F) -> (F, F) {
    let t58719 = -t57108 / 192.0 - t57110 / 32.0 - 7.0 / 144.0 * t57112 + 3.0 / 128.0 * t57114 - t57117 / 4.0 + t55562 + t57119 / 384.0 - 7.0 / 288.0 * t57121 + t57123 / 24.0 - t55564 + t55569 - t55570 - t55572;
    let t58730 = -t57127 / 2.0 + t57130 / 4.0 + t57132 / 24.0 - t57134 / 192.0 - t54293 - t54294 + t57138 / 12.0 + t55580 - 119.0 / 864.0 * t54305 - t57140 / 384.0 - 7.0 / 72.0 * t57142 - 7.0 / 24.0 * t57144 + 7.0 / 72.0 * t57146;
    (t58719, t58730)
}
