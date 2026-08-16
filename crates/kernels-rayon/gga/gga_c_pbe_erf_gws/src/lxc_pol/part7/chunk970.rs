//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 970/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk970(t401: f64, t5039: f64, t5030: f64, t190: f64, t212: f64, t367: f64, t16991: f64, t17001: f64, t17009: f64, t17011: f64, t17014: f64, t17016: f64, t17024: f64, t17030: f64, t17032: f64, t1714: f64, t25: f64, t5061: f64, t657: f64) -> f64 {
    let t17968 = t401 * t5039;
    let t17979 = t401 * t5030;
    let t17983 = 0.10864197530864197531e0_f64 * t190 * t367 * t212;
    let t17989 = 0.53333333333333333332e-1_f64 * t25 * t657 * t16991 - 0.10666666666666666667e0_f64 * t17968 + 0.79999999999999999998e-1_f64 * t25 * t1714 * t17001 - 0.88888888888888888888e-2_f64 * t25 * t1714 * t17009 - 0.17777777777777777778e-1_f64 * t25 * t5061 * t17014 + 0.17777777777777777778e-1_f64 * t17979 + t17983 - 0.9597777777777777778e-1_f64 * t17011 - 0.23994444444444444446e0_f64 * t17016 - 0.12957e1_f64 * t17024 - 0.28793333333333333333e0_f64 * t17030 + 0.95977777777777777777e-1_f64 * t17032;
    t17989
}
