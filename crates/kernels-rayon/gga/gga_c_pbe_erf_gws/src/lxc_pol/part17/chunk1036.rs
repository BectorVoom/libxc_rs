//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1036/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1036(t2083: f64, t3037: f64, t3259: f64, t3257: f64, t2084: f64, t816: f64, t2079: f64, t343: f64, t3220: f64, t3165: f64, t6: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9364 = t3037 * t2083;
    let t9365 = t9364 * t3259;
    let t9366 = t3257 * t9365;
    let t9370 = t2084 * t816;
    let t9371 = t343 * t2079 * t9370;
    let t9372 = t3220 * t9371;
    let t9375 = t6 * t3165;
    let t9376 = t254 * t9375;
    (t9364, t9365, t9366, t9370, t9371, t9372, t9375, t9376)
}
