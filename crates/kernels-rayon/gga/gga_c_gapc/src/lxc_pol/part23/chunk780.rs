//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 780/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk780(t633: f64, t8998: f64, t1700: f64, t5395: f64, t9067: f64, t5974: f64, t5967: f64, t9068: f64, t1510: f64, t1720: f64, t3108: f64, t1043: f64, t1668: f64) -> (f64, f64, f64, f64, f64) {
    let t9147 = t633 * t8998;
    let t9148 = t9147 * t1700;
    let t9150 = t5395 * t9067;
    let t9151 = t9150 * t5974;
    let t9153 = t9068 * t5967;
    let t9155 = t1720 * t1510;
    let t9156 = t3108 * t9155;
    let t9158 = t1043 * t1668;
    (t9148, t9151, t9153, t9156, t9158)
}
