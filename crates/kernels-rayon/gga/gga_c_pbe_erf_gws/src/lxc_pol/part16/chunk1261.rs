//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1261/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1261(t13944: f64, t2503: f64, t2409: f64, t28457: f64, t3965: f64, t2370: f64, t3958: f64, t53841: f64, t9284: f64, t4149: f64, t50998: f64, t9505: f64) -> (f64, f64, f64, f64) {
    let t53906 = t13944 * t2503;
    let t53910 = t3965 * t2409 * t28457;
    let t53923 = t3958 * t2370;
    let t53925 = t53923 * t53841 * t9284;
    let t53930 = t50998 * t4149 * t9505;
    (t53906, t53910, t53925, t53930)
}
