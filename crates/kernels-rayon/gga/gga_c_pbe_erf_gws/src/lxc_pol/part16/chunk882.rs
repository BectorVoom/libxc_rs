//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 882/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk882(t2660: f64, t2796: f64, t2800: f64, t1879: f64, t1033: f64, t1726: f64, t1733: f64, t209: f64, t184: f64, t1024: f64, t7593: f64, t7595: f64, t7597: f64, t7599: f64, t7601: f64, t7603: f64, t7605: f64, t7607: f64, t7609: f64, t7613: f64, t7615: f64, t7617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7619 = 16.0_f64 / 45.0_f64 * t2660 * t2796;
    let t7621 = 8.0_f64 / 15.0_f64 * t2660 * t2800;
    let t7623 = 16.0_f64 / 45.0_f64 * t1879 * t2796;
    let t7625 = 2.0_f64 / 15.0_f64 * t1033 * t1726;
    let t7626 = t1733 * t209;
    let t7627 = t7626 * t184;
    let t7629 = 4.0_f64 / 15.0_f64 * t7627 * t1024;
    let t7630 = t7593 + t7595 + t7597 + t7599 + t7601 + t7603 + t7605 + t7607 + t7609 - t7613 + t7615 + t7617 + t7619 + t7621 + t7623 - t7625 + t7629;
    (t7619, t7621, t7623, t7625, t7629, t7630)
}
