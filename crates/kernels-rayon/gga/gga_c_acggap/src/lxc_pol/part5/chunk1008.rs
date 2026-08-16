//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1008/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1008(t1162: f64, t16986: f64, t4452: f64, t5014: f64, t997: f64, t1352: f64, t3700: f64, t1181: f64, t12936: f64, t3655: f64, t4643: f64, t3044: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16987 = t16986 * t1162;
    let t16988 = t16987 * t4452;
    let t16990 = t997 * t5014;
    let t16992 = t3700 * t1352;
    let t16996 = t12936 * t1181 * t4643 * t3655;
    let t17000 = t12936 * t1181 * t535 * t3044;
    (t16987, t16988, t16990, t16992, t16996, t17000)
}
