//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 679/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk679(t5451: f64, t714: f64, t1791: f64, t1793: f64, t617: f64, t1621: f64, t1620: f64, t1627: f64, t1631: f64, t1893: f64, t155: f64, t641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5452 = t5451 * t714;
    let t5454 = t1791 * t1793;
    let t5455 = t5454 * t617;
    let t5456 = t1621 * t5455;
    let t5458 = 8.0_f64 / 5.0_f64 * t1620 * t5456;
    let t5459 = t1627 * t1631;
    let t5460 = 16.0_f64 / 45.0_f64 * t5459;
    let t5462 = 8.0_f64 / 15.0_f64 * t1627 * t1893;
    let t5463 = t155 * t641;
    (t5452, t5454, t5455, t5456, t5458, t5460, t5462, t5463)
}
