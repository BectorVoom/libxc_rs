//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1311/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1311(t50970: f64, t50972: f64, t52159: f64, t52167: f64, t53038: f64, t53053: f64, t53058: f64, t53065: f64, t53072: f64, t53126: f64, t54942: f64, t54946: f64, t54952: f64, t54957: f64, t54962: f64, t6793: f64) -> f64 {
    let t54969 = -t54942 + t53038 / 96.0_f64 + t53053 / 384.0_f64 + t53058 / 192.0_f64 - t54946 - t53065 / 384.0_f64 + 7.0_f64 / 36.0_f64 * t50970 - 7.0_f64 / 1152.0_f64 * t50972 + t6793 * t54952 / 24.0_f64 + t6793 * t54957 / 24.0_f64 + t6793 * t54962 / 24.0_f64 + t53072 / 96.0_f64 + 35.0_f64 / 108.0_f64 * t52159 - 7.0_f64 / 72.0_f64 * t52167 - t53126 / 12.0_f64;
    t54969
}
