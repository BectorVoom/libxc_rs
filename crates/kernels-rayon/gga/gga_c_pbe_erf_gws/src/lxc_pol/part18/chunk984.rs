//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 984/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk984(t2593: f64, t2612: f64, t1627: f64, t3519: f64, t643: f64, t9801: f64, t642: f64, t639: f64, t3523: f64, t1791: f64, t3390: f64, t617: f64) -> (f64, f64, f64, f64, f64) {
    let t11122 = 16.0_f64 / 45.0_f64 * t2612 * t2593;
    let t11124 = 4.0_f64 / 45.0_f64 * t1627 * t3519;
    let t11125 = t643 * t9801;
    let t11126 = t642 * t11125;
    let t11128 = 4.0_f64 / 45.0_f64 * t639 * t11126;
    let t11130 = 4.0_f64 / 27.0_f64 * t1627 * t3523;
    let t11131 = t1791 * t3390;
    let t11132 = t11131 * t617;
    (t11122, t11124, t11128, t11130, t11132)
}
