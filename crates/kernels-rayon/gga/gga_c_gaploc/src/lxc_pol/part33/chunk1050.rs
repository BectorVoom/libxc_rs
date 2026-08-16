//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1050/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1050(t10531: f64, t1433: f64, t539: f64, t599: f64, t4786: f64, t6715: f64, t1410: f64, t6295: f64, t900: f64, t1339: f64, t20013: f64, t1415: f64, t6834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20796 = t1433 * t10531;
    let t20800 = t539 * t599;
    let t20827 = t4786 * t6715;
    let t20843 = t1410 * t599;
    let t20887 = t900 * t6295;
    let t20896 = t1339 * t20013;
    let t20900 = t1415 * t6834;
    (t20796, t20800, t20827, t20843, t20887, t20896, t20900)
}
