//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 881/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk881(t1648: f64, t2622: f64, t2572: f64, t7011: f64, t4913: f64, t2705: f64, t422: f64, t7194: f64, t1620: f64, t1812: f64, t7527: f64, t1882: f64, t2790: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7605 = 16.0_f64 / 45.0_f64 * t1648 * t2622;
    let t7607 = 16.0_f64 / 45.0_f64 * t7011 * t2572;
    let t7609 = 16.0_f64 / 45.0_f64 * t4913 * t2572;
    let t7610 = t2705 * t422;
    let t7611 = t7194 * t7610;
    let t7613 = 16.0_f64 / 45.0_f64 * t1620 * t7611;
    let t7615 = 16.0_f64 / 45.0_f64 * t7527 * t1812;
    let t7617 = 16.0_f64 / 45.0_f64 * t2790 * t1882;
    (t7605, t7607, t7609, t7613, t7615, t7617)
}
