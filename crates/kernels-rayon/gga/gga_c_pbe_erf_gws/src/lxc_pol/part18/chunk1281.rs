//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1281/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1281(t2376: f64, t2408: f64, t2409: f64, t3717: f64, t4052: f64, t53093: f64, t53099: f64, t53155: f64, t53177: f64, t53179: f64, t53220: f64, t56228: f64, t56236: f64, t56240: f64, t56242: f64, t56250: f64, t56252: f64, t56255: f64, t56257: f64, t8793: f64) -> f64 {
    let t56259 = t8793 * t53220 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t56228 + t2408 * t2409 * t2376 * t4052 * t3717 / 48.0_f64 - t56236 / 12.0_f64 + t56240 / 1536.0_f64 - t53093 - t53099 + 7.0_f64 / 1152.0_f64 * t56242 - t53155 + t56250 / 384.0_f64 - 7.0_f64 / 72.0_f64 * t56252 + 5.0_f64 / 192.0_f64 * t56255 + 7.0_f64 / 48.0_f64 * t56257 - t53177 - t53179;
    t56259
}
