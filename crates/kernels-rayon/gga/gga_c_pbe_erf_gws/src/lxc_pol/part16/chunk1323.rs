//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1323/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1323(t20091: f64, t4209: f64, t53577: f64, t53583: f64, t53597: f64, t4110: f64, t6126: f64, t14185: f64, t3066: f64, t3068: f64, t3207: f64, t52331: f64, t53553: f64, t53562: f64, t53564: f64, t53567: f64, t53572: f64, t53579: f64, t53581: f64, t53595: f64, t9213: f64, t9283: f64) -> f64 {
    let t55243 = t20091 * t4209;
    let t55248 = 7.0_f64 / 72.0_f64 * t53577;
    let t55251 = 7.0_f64 / 576.0_f64 * t53583;
    let t55258 = 7.0_f64 / 288.0_f64 * t53597;
    let t55259 = t6126 * t4110;
    let t55264 = t53553 / 384.0_f64 - t53562 / 384.0_f64 + 35.0_f64 / 216.0_f64 * t55243 - t53564 / 24.0_f64 + t53567 / 24.0_f64 - t53572 / 12.0_f64 - t55248 - t53579 / 24.0_f64 - t53581 / 24.0_f64 - t55251 - 7.0_f64 / 72.0_f64 * t52331 - 5.0_f64 / 64.0_f64 * t53595 + t3207 * t9283 * t14185 * t9213 / 8.0_f64 - t55258 - t3066 * t9283 * t55259 * t3068 / 8.0_f64;
    t55264
}
