//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 638/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk638(t4991: f64, t592: f64, t587: f64, t1651: f64, t1897: f64, t1630: f64, t1892: f64, t639: f64, t1641: f64, t50: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4992 = t4991 * t592;
    let t4993 = t587 * t4992;
    let t4994 = 8.0_f64 / 135.0_f64 * t4993;
    let t4995 = t1651 * t1897;
    let t4996 = t587 * t4995;
    let t4997 = 16.0_f64 / 45.0_f64 * t4996;
    let t4998 = t1630 * t1892;
    let t4999 = t639 * t4998;
    let t5000 = 16.0_f64 / 45.0_f64 * t4999;
    let t5002 = 1.0_f64 / t1641 / t50;
    (t4992, t4994, t4995, t4997, t4998, t5000, t5002)
}
