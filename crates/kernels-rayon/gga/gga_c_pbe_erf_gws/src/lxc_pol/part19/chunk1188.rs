//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1188/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1188(t1161: f64, t4227: f64, t2409: f64, t3067: f64, t14506: f64, t14520: f64, t15218: f64, t15220: f64, t15222: f64, t15224: f64, t15226: f64, t15228: f64, t15230: f64, t15232: f64, t15234: f64, t15236: f64, t15238: f64, t15241: f64, t15243: f64, t15245: f64) -> (f64, f64, f64) {
    let t15443 = t4227 * t1161;
    let t15445 = t2409 * t3067 * t15443;
    let t15466 = t15218 / 384.0_f64 + t15220 / 48.0_f64 - t15222 / 48.0_f64 + t15224 / 384.0_f64 + t15226 / 48.0_f64 - t15228 / 24.0_f64 - t15230 / 384.0_f64 + t15232 / 128.0_f64 - t15234 / 384.0_f64 + t15236 / 12.0_f64 - t15238 / 12.0_f64 - t15241 / 48.0_f64 - t15243 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t14506 - 7.0_f64 / 72.0_f64 * t14520 + t15245 / 48.0_f64;
    (t15443, t15445, t15466)
}
