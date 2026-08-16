//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1099/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1099(t2409: f64, t4400: f64, t3965: f64, t1192: f64, t2074: f64, t2376: f64, t859: f64, t940: f64) -> (f64, f64, f64, f64, f64) {
    let t13976 = t2409 * t4400;
    let t13977 = t3965 * t13976;
    let t13979 = t1192 * t2074;
    let t13981 = t2409 * t2376 * t13979;
    let t13984 = t859 * t940;
    (t13976, t13977, t13979, t13981, t13984)
}
