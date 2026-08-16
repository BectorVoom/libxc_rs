//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1326/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1326(t53725: f64, t53727: f64, t1206: f64, t15082: f64, t3189: f64, t3207: f64, t335: f64, t338: f64, t4111: f64, t51564: f64, t53677: f64, t53689: f64, t53691: f64, t53693: f64, t53695: f64, t53700: f64, t53713: f64, t53715: f64, t53721: f64, t8804: f64, t892: f64, t9283: f64) -> f64 {
    let t55344 = 7.0_f64 / 72.0_f64 * t53725;
    let t55345 = 7.0_f64 / 1152.0_f64 * t53727;
    let t55350 = -t53677 / 24.0_f64 + 7.0_f64 / 576.0_f64 * t51564 - t53689 / 24.0_f64 - t53691 / 48.0_f64 - t53693 / 12.0_f64 + t53695 / 24.0_f64 - t53700 / 48.0_f64 - t53713 / 256.0_f64 + t53715 / 48.0_f64 - t53721 / 768.0_f64 - t3207 * t9283 * t4111 * t3189 / 8.0_f64 - t3207 * t9283 * t1206 * t8804 / 8.0_f64 - t55344 + t55345 - t335 * t338 * t892 * t15082 / 48.0_f64;
    t55350
}
