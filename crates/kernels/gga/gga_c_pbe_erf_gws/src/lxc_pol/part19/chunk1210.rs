//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1210/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1210<F: Float>(t54352: F, t54356: F, t54381: F, t55608: F, t55620: F, t57195: F, t57197: F, t57199: F, t57201: F, t57204: F, t57206: F, t57208: F, t57210: F, t52715: F, t55633: F, t55634: F, t57213: F, t57216: F, t57219: F, t57223: F, t57225: F, t57227: F, t57229: F, t57231: F, t57233: F, t57235: F) -> (F, F) {
    let t58765 = -119.0 / 432.0 * t54352 + t55608 - 35.0 / 54.0 * t54356 + t55620 - t57195 / 192.0 - t57197 / 96.0 - t57199 / 96.0 - 35.0 / 108.0 * t54381 + 7.0 / 144.0 * t57201 + t57204 / 12.0 - 7.0 / 144.0 * t57206 + t57208 / 12.0 + t57210 / 8.0;
    let t58776 = -t52715 + 7.0 / 288.0 * t57213 + t55633 - t55634 + t57216 / 48.0 - t57219 / 24.0 - t57223 / 48.0 + t57225 / 32.0 + t57227 / 192.0 + t57229 / 24.0 - t57231 / 192.0 + t57233 / 24.0 + 5.0 / 96.0 * t57235;
    (t58765, t58776)
}
