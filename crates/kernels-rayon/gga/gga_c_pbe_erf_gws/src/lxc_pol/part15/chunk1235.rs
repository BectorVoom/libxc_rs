//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1235/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1235(t14404: f64, t20113: f64, t50970: f64, t50972: f64, t51890: f64, t53028: f64, t53034: f64, t53038: f64, t53042: f64, t53047: f64, t53053: f64, t53058: f64, t53061: f64, t53065: f64, t53072: f64, t53075: f64, t6793: f64, t8793: f64) -> f64 {
    let t53078 = -t53028 - t8793 * t51890 / 16.0_f64 + t20113 * t14404 / 48.0_f64 + t6793 * t53034 / 24.0_f64 + t53038 / 192.0_f64 + t6793 * t53042 / 24.0_f64 + t6793 * t53047 / 24.0_f64 + t53053 / 768.0_f64 + t53058 / 384.0_f64 - t53061 - t53065 / 768.0_f64 + 7.0_f64 / 72.0_f64 * t50970 - 7.0_f64 / 2304.0_f64 * t50972 + t53072 / 192.0_f64 + t6793 * t53075 / 24.0_f64;
    t53078
}
