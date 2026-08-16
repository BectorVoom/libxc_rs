//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 928/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk928(t1621: f64, t1791: f64, t5097: f64, t639: f64, t661: f64, t1620: f64, t617: f64, t649: f64, t1672: f64, t1794: f64, t211: f64, t5105: f64, t633: f64) -> (f64, f64, f64, f64) {
    let t17354 = 16.0_f64 / 15.0_f64 * t639 * t1621 * t1791 * t5097 * t661;
    let t17359 = 16.0_f64 / 15.0_f64 * t1620 * t1621 * t649 * t5097 * t617;
    let t17361 = t211 * t1672 * t1794;
    let t17362 = 16.0_f64 / 45.0_f64 * t17361;
    let t17363 = t633 * t5105;
    (t17354, t17359, t17362, t17363)
}
