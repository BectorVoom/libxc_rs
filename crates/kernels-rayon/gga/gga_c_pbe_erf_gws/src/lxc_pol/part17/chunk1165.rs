//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1165/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1165(t3062: f64, t3959: f64, t14128: f64, t14130: f64, t14777: f64, t14779: f64, t14782: f64, t14785: f64, t14788: f64, t14793: f64, t14800: f64, t14803: f64, t14806: f64, t14809: f64, t2408: f64, t3066: f64) -> f64 {
    let t14812 = t3959 * t3062;
    let t14814 = t14777 / 1536.0_f64 + 7.0_f64 / 288.0_f64 * t14779 - t14782 / 96.0_f64 - t14785 / 384.0_f64 - t14788 / 96.0_f64 - 7.0_f64 / 288.0_f64 * t14128 - t3066 * t14793 / 16.0_f64 - 7.0_f64 / 288.0_f64 * t14130 + t14800 / 1536.0_f64 - t2408 * t14803 / 24.0_f64 + t14806 / 48.0_f64 + t2408 * t14809 / 48.0_f64 + t14812 / 48.0_f64;
    t14814
}
