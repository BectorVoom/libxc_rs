//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1116/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1116(t14222: f64, t14239: f64, t898: f64, t338: f64, t353: f64, t1205: f64, t2182: f64, t2376: f64, t2409: f64, t13778: f64, t13785: f64, t13789: f64, t13794: f64, t13801: f64, t13804: f64, t13809: f64, t13813: f64, t13818: f64, t14182: f64, t14188: f64, t14193: f64, t14198: f64, t14202: f64, t2384: f64, t2408: f64, t3207: f64, t335: f64, t4083: f64, t4385: f64, t6793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14240 = t14222 + t14239;
    let t14241 = t898 * t14240;
    let t14243 = t338 * t353 * t14241;
    let t14250 = t1205 * t2182;
    let t14252 = t2409 * t2376 * t14250;
    let t14257 = -t13778 / 96.0_f64 - t13785 / 384.0_f64 + t6793 * t14182 / 24.0_f64 + t6793 * t14188 / 24.0_f64 + t4385 * t14193 / 96.0_f64 - t2384 * t4083 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t14198 + t2408 * t14202 / 48.0_f64 - t13789 / 1536.0_f64 - t335 * t14243 / 96.0_f64 - t13794 / 12.0_f64 + t13801 / 768.0_f64 + t13804 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t13809 - t3207 * t14252 / 16.0_f64 + t13813 / 48.0_f64 + t13818 / 48.0_f64;
    (t14240, t14241, t14243, t14250, t14252, t14257)
}
