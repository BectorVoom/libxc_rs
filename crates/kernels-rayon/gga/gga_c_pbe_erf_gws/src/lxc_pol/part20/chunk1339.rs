//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1339/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1339(t54258: f64, t54260: f64, t57082: f64, t57086: f64, t57088: f64, t57090: f64, t57092: f64, t57094: f64, t57096: f64, t57098: f64, t57100: f64, t57102: f64, t57104: f64) -> f64 {
    let t57106 = -t57082 / 768.0_f64 + t57086 / 48.0_f64 - t57088 / 24.0_f64 - t57090 / 96.0_f64 - t57092 / 768.0_f64 - 5.0_f64 / 192.0_f64 * t57094 + t57096 / 96.0_f64 + t57098 / 48.0_f64 + t54258 + t57100 / 96.0_f64 - t57102 / 96.0_f64 - t54260 + 7.0_f64 / 1152.0_f64 * t57104;
    t57106
}
