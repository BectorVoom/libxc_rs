//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1322/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1322(t22336: f64, t4083: f64, t14883: f64, t9270: f64, t14959: f64, t4414: f64, t53545: f64, t14185: f64, t14958: f64, t2408: f64, t29751: f64, t3060: f64, t3212: f64, t51505: f64, t51507: f64, t51509: f64, t52191: f64, t53531: f64, t53537: f64, t53542: f64, t53549: f64, t8754: f64, t9283: f64) -> f64 {
    let t55212 = 7.0_f64 / 144.0_f64 * t22336 * t4083;
    let t55218 = 7.0_f64 / 24.0_f64 * t9270 * t14883;
    let t55228 = 7.0_f64 / 36.0_f64 * t4414 * t14959;
    let t55238 = 7.0_f64 / 288.0_f64 * t53545;
    let t55240 = t55212 + t53531 / 12.0_f64 - 7.0_f64 / 144.0_f64 * t51505 - 7.0_f64 / 1152.0_f64 * t51507 - t53537 / 1536.0_f64 + t55218 - t2408 * t29751 * t14958 / 12.0_f64 + t53542 / 768.0_f64 - t2408 * t9283 * t52191 * t3060 / 12.0_f64 + t55228 - 119.0_f64 / 3456.0_f64 * t51509 - t2408 * t9283 * t52191 * t3212 / 12.0_f64 - t2408 * t9283 * t14185 * t8754 / 12.0_f64 - t55238 - 5.0_f64 / 384.0_f64 * t53549;
    t55240
}
