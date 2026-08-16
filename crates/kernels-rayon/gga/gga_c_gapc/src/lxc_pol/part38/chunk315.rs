//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 315/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk315(t1302: f64, t1303: f64, t106: f64, t78: f64, t14: f64, t60: f64, t159: f64, t88: f64, t108: f64, t348: f64, t1147: f64, t391: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1304 = t1302 * t1303;
    let t1308 = t78 * t106;
    let t1312 = t60 * t14;
    let t1319 = t159 * t88;
    let t1320 = t348 * t108;
    let t1326 = t391 * t1147;
    (t1304, t1308, t1312, t1319, t1320, t1326)
}
