//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1237/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1237(t1115: f64, t1185: f64, t13772: f64, t13849: f64, t13910: f64, t13929: f64, t13939: f64, t14576: f64, t2074: f64, t2182: f64, t2376: f64, t2408: f64, t2409: f64, t2498: f64, t27105: f64, t3066: f64, t3067: f64, t3207: f64, t34963: f64, t4182: f64, t50967: f64, t53083: f64, t53093: f64, t53099: f64, t53126: f64, t53131: f64, t6793: f64, t810: f64, t8654: f64, t938: f64) -> f64 {
    let t53133 = -t6793 * t53083 / 12.0_f64 + t8654 * t27105 * t13929 / 24.0_f64 + t8654 * t1185 * t13910 / 24.0_f64 - t53093 - t3066 * t2409 * t34963 * t13849 / 16.0_f64 - t53099 + t2408 * t2409 * t2376 * t14576 * t810 / 24.0_f64 + t2408 * t2409 * t2376 * t4182 * t2074 / 48.0_f64 - t3207 * t2409 * t2376 * t4182 * t2182 / 16.0_f64 + t3066 * t2409 * t3067 * t14576 * t938 / 24.0_f64 - t2498 * t13939 / 48.0_f64 - t1115 * t50967 / 96.0_f64 - t2498 * t13772 / 48.0_f64 - t53126 / 24.0_f64 - t53131 / 1536.0_f64;
    t53133
}
