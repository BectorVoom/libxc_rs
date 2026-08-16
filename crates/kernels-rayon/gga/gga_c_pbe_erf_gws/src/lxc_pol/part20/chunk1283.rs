//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1283/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1283(t1193: f64, t353: f64, t3717: f64, t4386: f64, t13925: f64, t13930: f64, t2409: f64, t3066: f64, t3189: f64, t3207: f64, t34922: f64, t36129: f64, t36323: f64, t4164: f64, t4183: f64, t53187: f64, t53189: f64, t53199: f64, t53224: f64, t53231: f64, t53261: f64, t56265: f64, t56267: f64, t56269: f64, t56276: f64, t56282: f64, t6793: f64, t9283: f64) -> f64 {
    let t56287 = t4386 * t353 * t1193 * t3717;
    let t56294 = -t53187 - t53189 - t53199 + t3066 * t2409 * t36129 * t4164 / 24.0_f64 - t56265 / 768.0_f64 + 7.0_f64 / 48.0_f64 * t56267 - t56269 / 24.0_f64 + t34922 * t13925 / 96.0_f64 + t56276 / 768.0_f64 + t36323 * t13930 / 48.0_f64 + t6793 * t56282 / 24.0_f64 + t6793 * t56287 / 48.0_f64 - t53224 + t53231 - t3207 * t9283 * t4183 * t3189 / 8.0_f64 - t53261;
    t56294
}
