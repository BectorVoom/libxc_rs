//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1275/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1275(t2408: f64, t3060: f64, t50881: f64, t51084: f64, t51572: f64, t51595: f64, t53700: f64, t53704: f64, t53713: f64, t53715: f64, t53721: f64, t53726: f64, t53728: f64, t53730: f64, t53734: f64, t53736: f64, t53742: f64, t8629: f64, t9283: f64) -> f64 {
    let t53744 = -t53700 / 96.0_f64 - 35.0_f64 / 216.0_f64 * t51572 - t53704 - 7.0_f64 / 72.0_f64 * t51595 - t2408 * t9283 * t51084 * t3060 / 12.0_f64 - t53713 / 512.0_f64 + t53715 / 96.0_f64 - t53721 / 1536.0_f64 - t53726 + t53728 - t53730 + t53734 / 48.0_f64 - t53736 / 48.0_f64 + t8629 * t50881 / 96.0_f64 + t53742 / 1536.0_f64;
    t53744
}
