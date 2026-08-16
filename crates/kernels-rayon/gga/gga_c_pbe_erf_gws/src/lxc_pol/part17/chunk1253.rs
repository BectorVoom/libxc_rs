//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1253/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1253(t53424: f64, t3959: f64, t8766: f64, t13889: f64, t14397: f64, t14420: f64, t19895: f64, t2384: f64, t2388: f64, t2408: f64, t3066: f64, t3068: f64, t35566: f64, t4002: f64, t4052: f64, t51122: f64, t51126: f64, t51142: f64, t53395: f64, t53405: f64, t53407: f64, t53419: f64, t6126: f64, t6793: f64, t8634: f64, t9283: f64) -> f64 {
    let t53425 = 35.0_f64 / 576.0_f64 * t53424;
    let t53426 = t3959 * t8766;
    let t53429 = 7.0_f64 / 288.0_f64 * t51122 - t53395 / 768.0_f64 - t2384 * t14397 / 96.0_f64 - t8634 * t4002 / 48.0_f64 - t2388 * t14397 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t51126 + t53405 + t53407 - t3066 * t9283 * t6126 * t4052 * t3068 / 8.0_f64 - t2408 * t35566 * t13889 / 12.0_f64 + t19895 * t14420 / 48.0_f64 + t6793 * t53419 / 24.0_f64 - t53425 + t53426 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t51142;
    t53429
}
