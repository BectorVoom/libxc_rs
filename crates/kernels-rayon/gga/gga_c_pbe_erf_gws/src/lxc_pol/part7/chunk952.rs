//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 952/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk952(t1640: f64, t1791: f64, t1413: f64, t1642: f64, t1793: f64, t639: f64, t1620: f64, t4934: f64, t5141: f64, t5155: f64, t7877: f64, t17001: f64, t2677: f64) -> (f64, f64, f64, f64) {
    let t17646 = t1640 * t1791;
    let t17651 = 16.0_f64 / 9.0_f64 * t639 * t17646 * t1793 * t1642 * t1413;
    let t17653 = t1620 * t4934 * t5141;
    let t17654 = 64.0_f64 / 45.0_f64 * t17653;
    let t17656 = t1620 * t7877 * t5155;
    let t17657 = 64.0_f64 / 27.0_f64 * t17656;
    let t17660 = 16.0_f64 / 3.0_f64 * t639 * t2677 * t17001;
    (t17651, t17654, t17657, t17660)
}
