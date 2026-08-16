//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1127/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1127(t13919: f64, t3227: f64, t13917: f64, t1161: f64, t13888: f64, t353: f64, t859: f64, t1133: f64, t376: f64) -> (f64, f64, f64, f64, f64) {
    let t14415 = t13919 * t3227;
    let t14416 = t13917 * t14415;
    let t14418 = t13888 * t1161;
    let t14419 = t353 * t14418;
    let t14420 = t859 * t14419;
    let t14423 = t376 * t1133;
    (t14415, t14416, t14418, t14420, t14423)
}
