//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1256/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1256(t20154: f64, t2376: f64, t4155: f64, t814: f64, t50998: f64, t53447: f64, t6278: f64, t13917: f64, t14423: f64, t2157: f64, t2249: f64, t9640: f64) -> (f64, f64, f64) {
    let t53472 = t20154 * t2376 * t4155 * t814;
    let t53476 = t50998 * t53447 * t6278;
    let t53481 = t13917 * t2249 * t14423 * t2157 * t9640;
    (t53472, t53476, t53481)
}
