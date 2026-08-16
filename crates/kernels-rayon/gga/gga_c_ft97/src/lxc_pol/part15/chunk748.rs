//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 748/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk748(t20741: f64, t20872: f64, t20919: f64, t20977: f64, t160: f64, t20851: f64, t1023: f64, t1058: f64, t149: f64, t165: f64, t20527: f64, t20529: f64, t20678: f64, t20893: f64, t20898: f64, t20903: f64, t20908: f64, t20938: f64, t20973: f64, t4650: f64, t4720: f64, t4837: f64) -> (f64, f64, f64) {
    let t20979 = t20741 + t20872 + t20919 + t20977;
    let t20981 = t20851 * t160;
    let t20989 = -3.0_f64 * t1023 * t4837 - 3.0_f64 * t1058 * t4650 - 3.0_f64 * t1058 * t4720 - t149 * t20979 - t165 * t20527 - 2.0_f64 * t165 * t20529 - t165 * t20678 + 12.0_f64 * t20893 - 12.0_f64 * t20898 + 12.0_f64 * t20903 - 6.0_f64 * t20908 - 6.0_f64 * t20938 - 2.0_f64 * t20973 + 2.0_f64 * t20981;
    (t20979, t20981, t20989)
}
