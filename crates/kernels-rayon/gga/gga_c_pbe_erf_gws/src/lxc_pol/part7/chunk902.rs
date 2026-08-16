//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 902/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk902(t16999: f64, t17042: f64, t184: f64, t203: f64, t221: f64, t4913: f64, t4935: f64, t4879: f64, t1627: f64, t4930: f64, t4883: f64, t16945: f64, t16948: f64, t16953: f64, t16955: f64, t16957: f64, t16959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17047 = 2.0_f64 / 15.0_f64 * t203 * (t16999 + t17042) * t184 * t221;
    let t17048 = t4913 * t4935;
    let t17049 = 128.0_f64 / 45.0_f64 * t17048;
    let t17051 = 64.0_f64 / 15.0_f64 * t4913 * t4879;
    let t17053 = 32.0_f64 / 15.0_f64 * t1627 * t4930;
    let t17055 = 32.0_f64 / 15.0_f64 * t1627 * t4883;
    let t17056 = -t16945 - t16948 - t16953 + t16955 - t16957 - t16959 + t17047 + t17049 - t17051 + t17053 + t17055;
    (t17047, t17049, t17051, t17053, t17055, t17056)
}
