//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1211/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1211<F: Float>(t1115: F, t335: F, t338: F, t353: F, t52525: F, t54427: F, t54435: F, t55142: F, t55752: F, t55769: F, t55773: F, t57398: F, t57402: F, t57404: F, t57410: F, t57415: F, t57422: F, t57434: F, t58581: F, t58596: F, t58608: F, t58619: F, t58630: F, t58645: F, t58655: F, t58670: F, t58683: F, t58697: F, t58709: F, t58719: F, t58730: F, t58742: F, t58752: F, t58765: F, t58776: F, t8793: F, t898: F) -> (F,) {
    let t58797 = 7.0 / 288.0 * t58581 - 119.0 / 864.0 * t54427 - t335 * t338 * t353 * t898 * (t58596 + t58608 + t58619 + t58630 + t58645 + t58655 + t58670 + t58683 + t58697 + t58709 + t58719 + t58730 + t58742 + t58752 + t58765 + t58776) / 96.0 + t55752 - t57398 / 24.0 - t52525 + t54435 + t57402 / 12.0 + t57404 / 12.0 - t1115 * t55142 / 48.0 - t57410 / 96.0 - t57415 / 96.0 - t57422 / 768.0 - t57434 / 768.0 + t8793 * t55769 / 24.0 - t55773;
    (t58797,)
}
