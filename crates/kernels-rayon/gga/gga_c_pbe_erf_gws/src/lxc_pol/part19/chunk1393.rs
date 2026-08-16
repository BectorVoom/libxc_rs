//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1393/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1393(t1115: f64, t335: f64, t338: f64, t353: f64, t52525: f64, t54427: f64, t54435: f64, t55142: f64, t55752: f64, t55769: f64, t55773: f64, t57398: f64, t57402: f64, t57404: f64, t57410: f64, t57415: f64, t57422: f64, t57434: f64, t58581: f64, t58596: f64, t58608: f64, t58619: f64, t58630: f64, t58645: f64, t58655: f64, t58670: f64, t58683: f64, t58697: f64, t58709: f64, t58719: f64, t58730: f64, t58742: f64, t58752: f64, t58765: f64, t58776: f64, t8793: f64, t898: f64) -> f64 {
    let t58797 = 7.0_f64 / 288.0_f64 * t58581 - 119.0_f64 / 864.0_f64 * t54427 - t335 * t338 * t353 * t898 * (t58596 + t58608 + t58619 + t58630 + t58645 + t58655 + t58670 + t58683 + t58697 + t58709 + t58719 + t58730 + t58742 + t58752 + t58765 + t58776) / 96.0_f64 + t55752 - t57398 / 24.0_f64 - t52525 + t54435 + t57402 / 12.0_f64 + t57404 / 12.0_f64 - t1115 * t55142 / 48.0_f64 - t57410 / 96.0_f64 - t57415 / 96.0_f64 - t57422 / 768.0_f64 - t57434 / 768.0_f64 + t8793 * t55769 / 24.0_f64 - t55773;
    t58797
}
