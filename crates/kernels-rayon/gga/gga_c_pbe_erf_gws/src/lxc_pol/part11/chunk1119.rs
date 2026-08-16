//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1119/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1119(t3390: f64, t3469: f64, t4927: f64, t639: f64, t1033: f64, t12585: f64, t32093: f64, t1019: f64, t12452: f64, t18237: f64, t34500: f64, t43029: f64, t47832: f64, t47836: f64, t47839: f64, t47841: f64, t47844: f64) -> (f64, f64, f64, f64, f64) {
    let t47848 = 32.0_f64 / 15.0_f64 * t639 * t4927 * t3469 * t3390;
    let t47850 = 16.0_f64 / 5.0_f64 * t1033 * t12585;
    let t47851 = 16.0_f64 / 45.0_f64 * t32093;
    let t47855 = 16.0_f64 / 5.0_f64 * t12452 * t1019;
    let t47856 = -t47832 - t47836 + t47839 - t47841 - t47844 - t47848 - t47850 - t47851 + 8.0_f64 / 9.0_f64 * t43029 + 8.0_f64 / 3.0_f64 * t34500 - t47855 + t18237;
    (t47848, t47850, t47851, t47855, t47856)
}
